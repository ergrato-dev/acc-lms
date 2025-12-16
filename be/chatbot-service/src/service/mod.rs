//! # Service Module

pub mod chatbot_service;

pub use chatbot_service::{ChatbotService, ChatbotError, AIClient, AIResponse, Result};
