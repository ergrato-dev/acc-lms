# Video Content Protection Strategy

## Overview

This document describes the **public architecture** for video content protection in ACC-LMS.
The actual implementation details are maintained separately and are NOT included in the
public repository.

## Protection Layers

ACC-LMS implements a multi-layered approach to video protection:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        VIDEO PROTECTION ARCHITECTURE                        │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────┐   ┌─────────────┐   ┌─────────────┐   ┌─────────────┐     │
│  │   Layer 1   │   │   Layer 2   │   │   Layer 3   │   │   Layer 4   │     │
│  │  Streaming  │ → │  Encryption │ → │ Watermark   │ → │  Client     │     │
│  │  Protocol   │   │    (DRM)    │   │  (Forensic) │   │  Security   │     │
│  └─────────────┘   └─────────────┘   └─────────────┘   └─────────────┘     │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Layer 1: Adaptive Streaming Protocol

- **HLS (HTTP Live Streaming)** with encrypted segments
- Short-lived signed URLs (10-60 seconds validity)
- Geo-restriction capabilities
- Bandwidth adaptation

### Layer 2: Encryption / DRM

- Industry-standard DRM integration
- AES-128 encryption for HLS segments
- Key rotation policies
- Multi-DRM support for different platforms

### Layer 3: Forensic Watermarking

- Invisible watermarks embedded in video
- User-specific identification
- Survives common transformations (re-encoding, screen capture)
- Enables leak source identification

### Layer 4: Client-Side Security

- Secure video player implementation
- Anti-debugging measures
- Screenshot/screen recording detection
- DevTools detection

## Public API Endpoints

```
POST   /api/v1/videos/request-playback    # Request video playback session
GET    /api/v1/videos/:id/manifest        # Get HLS manifest (authenticated)
GET    /api/v1/videos/:id/key             # Get decryption key (authenticated)
POST   /api/v1/videos/report-violation    # Report suspected piracy
```

## Implementation Notes

> ⚠️ **IMPORTANT**: The actual implementation of these protection mechanisms
> is maintained in a **private repository** and is NOT included in the
> open-source version of ACC-LMS.

### Files NOT in Public Repository

The following directories/files are excluded from the public repository:

```
be/
├── drm/                      # DRM integration code
├── video-protection/         # Core protection logic
├── proprietary/              # Proprietary algorithms
└── services/
    └── media-service/
        └── src/
            ├── drm/          # DRM implementation
            └── watermark/    # Watermarking implementation
```

### For Contributors

If you're working on the video protection system:

1. Request access to the private implementation repository
2. Clone it as a submodule or separate directory
3. Never commit protection implementation details to the public repo
4. Use the interfaces defined in this document

## Integration Points

The public codebase provides interfaces that the private implementation must satisfy:

```rust
// Public interface (in public repo)
pub trait VideoProtectionService: Send + Sync {
    /// Generates a secure playback session
    async fn create_playback_session(
        &self,
        user_id: Uuid,
        video_id: Uuid,
        client_info: ClientInfo,
    ) -> Result<PlaybackSession, ApiError>;

    /// Validates and returns encryption key
    async fn get_encryption_key(
        &self,
        session_id: Uuid,
        segment_id: &str,
    ) -> Result<EncryptionKey, ApiError>;

    /// Checks if playback is authorized
    async fn validate_playback(
        &self,
        session_id: Uuid,
    ) -> Result<bool, ApiError>;
}

// Implementation is in private repo
```

## Security Considerations

### What We Can Share Publicly

- General architecture diagrams
- API endpoint specifications
- Data models and interfaces
- Integration patterns

### What Remains Private

- Encryption key derivation algorithms
- Watermarking injection techniques
- Anti-tampering implementations
- DRM license server configurations
- Obfuscation techniques

## Platform-Specific Considerations

| Platform                    | DRM Support | Notes                            |
| --------------------------- | ----------- | -------------------------------- |
| Web (Chrome, Firefox, Edge) | Widevine    | Most common                      |
| Web (Safari)                | FairPlay    | Requires Apple developer account |
| iOS                         | FairPlay    | Native integration               |
| Android                     | Widevine    | Native integration               |
| Desktop Apps                | Multi-DRM   | Electron with DRM support        |

## Related Documentation

- [Non-Functional Requirements](./non-functional-requirements.md) - Security requirements
- [Infrastructure](./architecture/infrastructure-traefik.md) - CDN configuration

---

_This document is part of the public ACC-LMS documentation. Implementation details
are maintained separately for security purposes._
