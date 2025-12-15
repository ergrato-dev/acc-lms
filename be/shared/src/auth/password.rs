//! # Password Hashing with Argon2id
//!
//! Secure password storage using the Argon2id algorithm.
//!
//! ## Why Argon2id?
//!
//! Argon2 won the [Password Hashing Competition](https://www.password-hashing.net/)
//! in 2015 and is recommended by [OWASP](https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html).
//!
//! There are three variants:
//!
//! | Variant | Resistance | Use Case |
//! |---------|------------|----------|
//! | Argon2d | GPU attacks | Cryptocurrency |
//! | Argon2i | Side-channel | General use |
//! | **Argon2id** | **Both** | **Passwords (recommended)** |
//!
//! We use **Argon2id** which combines the strengths of both variants.
//!
//! ## How Password Hashing Works
//!
//! ```text
//! ┌──────────────────────────────────────────────────────────────────────┐
//! │                        Password Hashing                              │
//! ├──────────────────────────────────────────────────────────────────────┤
//! │                                                                      │
//! │  "MyPassword123!"  ──►  Argon2id  ──►  $argon2id$v=19$m=65536...    │
//! │         │                   │                   │                   │
//! │    (plaintext)          (salt +            (PHC string:            │
//! │                         params)             algorithm +            │
//! │                                             version +              │
//! │                                             params +               │
//! │                                             salt +                 │
//! │                                             hash)                  │
//! └──────────────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Security Parameters (OWASP Recommended)
//!
//! Our implementation uses these parameters:
//!
//! | Parameter | Value | Purpose |
//! |-----------|-------|---------|
//! | Memory | 64 MiB | Makes attacks expensive (GPU memory) |
//! | Iterations | 3 | Time cost (slows brute force) |
//! | Parallelism | 4 | CPU lanes (matches typical cores) |
//! | Output | 32 bytes | Hash length |
//!
//! These settings balance security with acceptable response time (~100ms).
//!
//! ## PHC String Format
//!
//! We use the [PHC String Format](https://github.com/P-H-C/phc-string-format)
//! which is self-describing and includes:
//!
//! ```text
//! $argon2id$v=19$m=65536,t=3,p=4$<salt>$<hash>
//!   │         │    │      │   │   │       └── The hash output
//!   │         │    │      │   │   └────────── Random salt (base64)
//!   │         │    │      │   └────────────── Parallelism (4 lanes)
//!   │         │    │      └────────────────── Time cost (3 iterations)
//!   │         │    └───────────────────────── Memory (64 MiB)
//!   │         └────────────────────────────── Version (0x13 = 19)
//!   └──────────────────────────────────────── Algorithm identifier
//! ```
//!
//! ## Password Requirements (RF-AUTH-001)
//!
//! The `PasswordValidator` enforces:
//!
//! - Minimum 10 characters
//! - At least 1 uppercase letter
//! - At least 1 lowercase letter
//! - At least 1 digit
//! - At least 1 special character (!@#$%^&*)
//!
//! ## Usage Example
//!
//! ```rust,ignore
//! use shared::auth::password::{PasswordHasher, PasswordValidator};
//!
//! let hasher = PasswordHasher::new();
//!
//! // During registration
//! if let Err(errors) = PasswordValidator::validate(&password) {
//!     return Err(ApiError::ValidationError { ... });
//! }
//! let hash = hasher.hash(&password)?;
//! // Store hash in database
//!
//! // During login
//! if hasher.verify(&password, &stored_hash)? {
//!     // Password correct
//! } else {
//!     return Err(ApiError::InvalidCredentials);
//! }
//! ```
//!
//! ## Security Notes
//!
//! - **Never store plaintext passwords**
//! - **Each hash includes a unique salt** - Same password → different hashes
//! - **Verification is constant-time** - Prevents timing attacks
//! - **Hash updates** - If parameters change, re-hash on successful login
//!
//! ## Related Documentation
//!
//! - [OWASP Password Storage Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html)
//! - [`_docs/business/functional-requirements.md`] - RF-AUTH-001

use crate::errors::ApiError;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher as _, PasswordVerifier, SaltString},
    Argon2, Algorithm, Params, Version,
};

// =============================================================================
// Password Hasher
// =============================================================================

/// Secure password hashing service using Argon2id.
///
/// This service handles:
/// - Hashing passwords for storage
/// - Verifying passwords during login
///
/// ## Thread Safety
///
/// This type is `Clone` and `Send + Sync`, safe for concurrent use.
/// Each hash operation generates a new random salt.
///
/// ## Example
///
/// ```rust,ignore
/// let hasher = PasswordHasher::new();
///
/// // Hash a password
/// let hash = hasher.hash("MySecureP@ssw0rd!")?;
///
/// // Verify later
/// let is_valid = hasher.verify("MySecureP@ssw0rd!", &hash)?;
/// ```
#[derive(Clone)]
pub struct PasswordHasher {
    /// Pre-configured Argon2 instance
    argon2: Argon2<'static>,
}

impl Default for PasswordHasher {
    fn default() -> Self {
        Self::new()
    }
}

impl PasswordHasher {
    /// Creates a new hasher with OWASP-recommended parameters.
    ///
    /// ## Parameters Used
    ///
    /// - Memory: 64 MiB (protects against GPU attacks)
    /// - Iterations: 3 (time cost)
    /// - Parallelism: 4 lanes (uses multiple CPU cores)
    ///
    /// These parameters provide strong security while keeping
    /// verification time around 100ms on modern hardware.
    pub fn new() -> Self {
        // OWASP-recommended parameters for password hashing
        // See: https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html
        let params = Params::new(
            64 * 1024, // 64 MiB memory cost
            3,         // 3 iterations (time cost)
            4,         // 4 lanes (parallelism)
            None,      // Default output length (32 bytes)
        )
        .expect("Invalid Argon2 params"); // Safe: these params are always valid

        // Use Argon2id variant (v0x13 = version 19)
        let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

        Self { argon2 }
    }

    /// Hashes a password for secure storage.
    ///
    /// ## Process
    ///
    /// 1. Generate a cryptographically random salt
    /// 2. Apply Argon2id with configured parameters
    /// 3. Return the result in PHC string format
    ///
    /// ## Returns
    ///
    /// A PHC-formatted string containing the algorithm, version,
    /// parameters, salt, and hash. Example:
    ///
    /// ```text
    /// $argon2id$v=19$m=65536,t=3,p=4$<salt>$<hash>
    /// ```
    ///
    /// ## Errors
    ///
    /// Returns `ApiError::InternalError` if hashing fails (rare).
    pub fn hash(&self, password: &str) -> Result<String, ApiError> {
        // Generate a cryptographically secure random salt
        let salt = SaltString::generate(&mut OsRng);

        // Hash the password
        self.argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|e| ApiError::InternalError {
                message: format!("Password hashing failed: {}", e),
            })
    }

    /// Verifies a password against a stored hash.
    ///
    /// ## Process
    ///
    /// 1. Parse the PHC string to extract parameters and salt
    /// 2. Re-hash the provided password with the same salt
    /// 3. Compare in constant time (prevents timing attacks)
    ///
    /// ## Returns
    ///
    /// - `Ok(true)` - Password matches
    /// - `Ok(false)` - Password doesn't match
    /// - `Err(...)` - Invalid hash format or other error
    ///
    /// ## Security Note
    ///
    /// This uses constant-time comparison to prevent timing attacks.
    /// An attacker cannot determine how "close" a guess was.
    pub fn verify(&self, password: &str, hash: &str) -> Result<bool, ApiError> {
        // Parse the stored hash
        let parsed_hash = PasswordHash::new(hash).map_err(|e| ApiError::InternalError {
            message: format!("Invalid password hash format: {}", e),
        })?;

        // Verify with constant-time comparison
        match self.argon2.verify_password(password.as_bytes(), &parsed_hash) {
            Ok(()) => Ok(true),
            Err(argon2::password_hash::Error::Password) => Ok(false),
            Err(e) => Err(ApiError::InternalError {
                message: format!("Password verification failed: {}", e),
            }),
        }
    }
}

// Implement Debug manually to avoid exposing internal state
impl std::fmt::Debug for PasswordHasher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PasswordHasher").finish_non_exhaustive()
    }
}

// =============================================================================
// Password Validator
// =============================================================================

/// Validates password strength before hashing.
///
/// This ensures passwords meet minimum security requirements
/// as defined in RF-AUTH-001.
///
/// ## Requirements
///
/// | Requirement | Reason |
/// |-------------|--------|
/// | 10+ characters | Increases search space |
/// | 1+ uppercase | Increases character set |
/// | 1+ lowercase | Increases character set |
/// | 1+ digit | Increases character set |
/// | 1+ symbol | Increases character set |
///
/// ## Example
///
/// ```rust,ignore
/// match PasswordValidator::validate("weak") {
///     Ok(()) => println!("Password is strong enough"),
///     Err(errors) => {
///         for error in errors {
///             println!("- {}", error);
///         }
///     }
/// }
/// ```
pub struct PasswordValidator;

impl PasswordValidator {
    /// Validates that a password meets minimum strength requirements.
    ///
    /// ## Returns
    ///
    /// - `Ok(())` - Password meets all requirements
    /// - `Err(Vec<&str>)` - List of failed requirements
    ///
    /// ## Requirements (RF-AUTH-001)
    ///
    /// - Minimum 10 characters
    /// - At least 1 uppercase letter (A-Z)
    /// - At least 1 lowercase letter (a-z)
    /// - At least 1 digit (0-9)
    /// - At least 1 special character (!@#$%^&*)
    pub fn validate(password: &str) -> Result<(), Vec<&'static str>> {
        let mut errors = Vec::new();

        // Check minimum length
        if password.len() < 10 {
            errors.push("Password must be at least 10 characters long");
        }

        // Check for uppercase letter
        if !password.chars().any(|c| c.is_uppercase()) {
            errors.push("Password must contain at least one uppercase letter");
        }

        // Check for lowercase letter
        if !password.chars().any(|c| c.is_lowercase()) {
            errors.push("Password must contain at least one lowercase letter");
        }

        // Check for digit
        if !password.chars().any(|c| c.is_ascii_digit()) {
            errors.push("Password must contain at least one digit");
        }

        // Check for special character
        if !password.chars().any(|c| "!@#$%^&*".contains(c)) {
            errors.push("Password must contain at least one special character (!@#$%^&*)");
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_verify_correct_password() {
        let hasher = PasswordHasher::new();
        let password = "MySecureP@ssw0rd!";

        let hash = hasher.hash(password).unwrap();

        // Hash should be different from plaintext
        assert_ne!(hash, password);

        // Verification should pass with correct password
        assert!(hasher.verify(password, &hash).unwrap());
    }

    #[test]
    fn test_verify_wrong_password() {
        let hasher = PasswordHasher::new();
        let password = "MySecureP@ssw0rd!";
        let wrong_password = "WrongP@ssw0rd!";

        let hash = hasher.hash(password).unwrap();

        // Verification should fail with wrong password
        assert!(!hasher.verify(wrong_password, &hash).unwrap());
    }

    #[test]
    fn test_different_hashes_for_same_password() {
        let hasher = PasswordHasher::new();
        let password = "MySecureP@ssw0rd!";

        let hash1 = hasher.hash(password).unwrap();
        let hash2 = hasher.hash(password).unwrap();

        // Each hash should be unique due to random salt
        assert_ne!(hash1, hash2);

        // But both should verify correctly
        assert!(hasher.verify(password, &hash1).unwrap());
        assert!(hasher.verify(password, &hash2).unwrap());
    }

    #[test]
    fn test_hash_is_phc_format() {
        let hasher = PasswordHasher::new();
        let hash = hasher.hash("TestP@ssw0rd!").unwrap();

        // PHC string should start with algorithm identifier
        assert!(hash.starts_with("$argon2id$"));
    }

    #[test]
    fn test_password_validator_valid_password() {
        // This password meets all requirements
        assert!(PasswordValidator::validate("MyP@ssw0rd!").is_ok());
        assert!(PasswordValidator::validate("Str0ng&Pass").is_ok());
        assert!(PasswordValidator::validate("C0mpl3x!Pwd").is_ok());
    }

    #[test]
    fn test_password_validator_too_short() {
        let result = PasswordValidator::validate("Short1!");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains(&"Password must be at least 10 characters long"));
    }

    #[test]
    fn test_password_validator_missing_uppercase() {
        let result = PasswordValidator::validate("myp@ssw0rd!");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains(&"Password must contain at least one uppercase letter"));
    }

    #[test]
    fn test_password_validator_missing_lowercase() {
        let result = PasswordValidator::validate("MYP@SSW0RD!");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains(&"Password must contain at least one lowercase letter"));
    }

    #[test]
    fn test_password_validator_missing_digit() {
        let result = PasswordValidator::validate("MyP@ssword!");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains(&"Password must contain at least one digit"));
    }

    #[test]
    fn test_password_validator_missing_symbol() {
        let result = PasswordValidator::validate("MyPassw0rd1");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains(&"Password must contain at least one special character (!@#$%^&*)"));
    }

    #[test]
    fn test_password_validator_multiple_errors() {
        let result = PasswordValidator::validate("short");
        assert!(result.is_err());
        
        let errors = result.unwrap_err();
        // Should have multiple errors
        assert!(errors.len() > 1);
    }
}

