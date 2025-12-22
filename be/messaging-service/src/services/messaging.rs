//! # Messaging Service
//!
//! Business logic for conversations and messaging.

use uuid::Uuid;

use crate::api::dto::*;
use crate::domain::errors::MessagingError;
use crate::repository::MessagingRepository;

/// Service for messaging business logic.
#[derive(Clone)]
pub struct MessagingService {
    repository: MessagingRepository,
}

impl MessagingService {
    /// Create a new service instance.
    pub fn new(repository: MessagingRepository) -> Self {
        Self { repository }
    }

    // =========================================================================
    // Conversation Operations
    // =========================================================================

    /// List conversations for a user.
    pub async fn list_conversations(
        &self,
        user_id: Uuid,
        page: i32,
        per_page: i32,
        conversation_type: Option<String>,
        include_archived: bool,
    ) -> Result<PaginatedConversationsResponse, MessagingError> {
        let offset = (page - 1) * per_page;

        let conversations = self.repository.find_conversations_for_user(
            user_id,
            per_page,
            offset,
            conversation_type.as_deref(),
            include_archived,
        ).await?;

        let total = self.repository.count_conversations_for_user(
            user_id,
            conversation_type.as_deref(),
            include_archived,
        ).await?;

        let total_pages = ((total as f64) / (per_page as f64)).ceil() as i32;

        let mut items = Vec::new();
        for conv in conversations {
            let participants = self.repository.find_participants(conv.conversation_id).await?;
            let unread_count = self.repository.get_unread_count(user_id, conv.conversation_id).await?;
            let settings = self.repository.get_participant_settings(user_id, conv.conversation_id).await?;

            let participant_responses: Vec<ParticipantResponse> = participants.iter().map(|p| {
                ParticipantResponse {
                    user_id: p.user_id,
                    name: format!("User {}", p.user_id), // Would be fetched from user service
                    avatar_url: None,
                    role: p.role.clone(),
                    is_online: false,
                    last_seen: None,
                }
            }).collect();

            items.push(ConversationResponse {
                conversation_id: conv.conversation_id,
                conversation_type: conv.conversation_type.clone(),
                title: conv.title.clone(),
                course_id: conv.course_id,
                participants: participant_responses,
                last_message: None, // Would be fetched separately
                unread_count,
                is_muted: settings.as_ref().map(|s| s.is_muted).unwrap_or(false),
                is_archived: settings.as_ref().map(|s| s.is_archived).unwrap_or(false),
                created_at: conv.created_at,
                updated_at: conv.updated_at,
            });
        }

        Ok(PaginatedConversationsResponse {
            items,
            total,
            page,
            per_page,
            total_pages,
        })
    }

    /// Create a new conversation.
    pub async fn create_conversation(
        &self,
        creator_id: Uuid,
        conversation_type: &str,
        title: Option<String>,
        course_id: Option<Uuid>,
        participant_ids: &[Uuid],
    ) -> Result<ConversationResponse, MessagingError> {
        // For direct conversations, check if one already exists
        if conversation_type == "direct" && participant_ids.len() == 1 {
            if let Some(existing) = self.repository.find_direct_conversation(
                creator_id,
                participant_ids[0],
            ).await? {
                return self.get_conversation(creator_id, existing.conversation_id).await;
            }
        }

        // Create the conversation
        let conversation = self.repository.create_conversation(
            conversation_type,
            title.as_deref(),
            course_id,
            creator_id,
        ).await?;

        // Add creator as participant
        self.repository.add_participant(
            conversation.conversation_id,
            creator_id,
            "member",
        ).await?;

        // Add other participants
        for participant_id in participant_ids {
            if *participant_id != creator_id {
                self.repository.add_participant(
                    conversation.conversation_id,
                    *participant_id,
                    "member",
                ).await?;
            }
        }

        self.get_conversation(creator_id, conversation.conversation_id).await
    }

    /// Get a specific conversation.
    pub async fn get_conversation(
        &self,
        user_id: Uuid,
        conversation_id: Uuid,
    ) -> Result<ConversationResponse, MessagingError> {
        // Verify user is a participant
        let is_participant = self.repository.is_participant(user_id, conversation_id).await?;
        if !is_participant {
            return Err(MessagingError::Forbidden("Not a participant in this conversation".into()));
        }

        let conversation = self.repository.find_conversation_by_id(conversation_id).await?
            .ok_or(MessagingError::NotFound("Conversation not found".into()))?;

        let participants = self.repository.find_participants(conversation_id).await?;
        let unread_count = self.repository.get_unread_count(user_id, conversation_id).await?;
        let settings = self.repository.get_participant_settings(user_id, conversation_id).await?;

        let participant_responses: Vec<ParticipantResponse> = participants.iter().map(|p| {
            ParticipantResponse {
                user_id: p.user_id,
                name: format!("User {}", p.user_id),
                avatar_url: None,
                role: p.role.clone(),
                is_online: false,
                last_seen: None,
            }
        }).collect();

        Ok(ConversationResponse {
            conversation_id: conversation.conversation_id,
            conversation_type: conversation.conversation_type,
            title: conversation.title,
            course_id: conversation.course_id,
            participants: participant_responses,
            last_message: None,
            unread_count,
            is_muted: settings.as_ref().map(|s| s.is_muted).unwrap_or(false),
            is_archived: settings.as_ref().map(|s| s.is_archived).unwrap_or(false),
            created_at: conversation.created_at,
            updated_at: conversation.updated_at,
        })
    }

    /// Toggle mute status for a conversation.
    pub async fn toggle_mute(
        &self,
        user_id: Uuid,
        conversation_id: Uuid,
        muted: bool,
    ) -> Result<(), MessagingError> {
        let is_participant = self.repository.is_participant(user_id, conversation_id).await?;
        if !is_participant {
            return Err(MessagingError::Forbidden("Not a participant in this conversation".into()));
        }

        self.repository.update_participant_settings(
            user_id,
            conversation_id,
            Some(muted),
            None,
        ).await?;

        Ok(())
    }

    /// Toggle archive status for a conversation.
    pub async fn toggle_archive(
        &self,
        user_id: Uuid,
        conversation_id: Uuid,
        archived: bool,
    ) -> Result<(), MessagingError> {
        let is_participant = self.repository.is_participant(user_id, conversation_id).await?;
        if !is_participant {
            return Err(MessagingError::Forbidden("Not a participant in this conversation".into()));
        }

        self.repository.update_participant_settings(
            user_id,
            conversation_id,
            None,
            Some(archived),
        ).await?;

        Ok(())
    }

    // =========================================================================
    // Message Operations
    // =========================================================================

    /// List messages in a conversation.
    pub async fn list_messages(
        &self,
        user_id: Uuid,
        conversation_id: Uuid,
        page: i32,
        per_page: i32,
        before: Option<Uuid>,
        after: Option<Uuid>,
    ) -> Result<PaginatedMessagesResponse, MessagingError> {
        let is_participant = self.repository.is_participant(user_id, conversation_id).await?;
        if !is_participant {
            return Err(MessagingError::Forbidden("Not a participant in this conversation".into()));
        }

        let offset = (page - 1) * per_page;

        let messages = self.repository.find_messages(
            conversation_id,
            per_page,
            offset,
            before,
            after,
        ).await?;

        let total = self.repository.count_messages(conversation_id).await?;
        let total_pages = ((total as f64) / (per_page as f64)).ceil() as i32;
        let has_more = page < total_pages;

        let items: Vec<MessageResponse> = messages.iter().map(|m| {
            MessageResponse {
                message_id: m.message_id,
                conversation_id: m.conversation_id,
                sender: SenderResponse {
                    user_id: m.sender_id,
                    name: format!("User {}", m.sender_id),
                    avatar_url: None,
                },
                content: m.content.clone(),
                message_type: m.message_type.clone(),
                reply_to_id: m.reply_to_id,
                is_edited: m.is_edited,
                is_deleted: m.is_deleted,
                created_at: m.created_at,
                updated_at: m.updated_at,
            }
        }).collect();

        Ok(PaginatedMessagesResponse {
            items,
            total,
            page,
            per_page,
            total_pages,
            has_more,
        })
    }

    /// Send a message.
    pub async fn send_message(
        &self,
        sender_id: Uuid,
        conversation_id: Uuid,
        content: &str,
        message_type: &str,
        reply_to_id: Option<Uuid>,
    ) -> Result<MessageSentResponse, MessagingError> {
        let is_participant = self.repository.is_participant(sender_id, conversation_id).await?;
        if !is_participant {
            return Err(MessagingError::Forbidden("Not a participant in this conversation".into()));
        }

        // Validate reply_to_id if provided
        if let Some(reply_id) = reply_to_id {
            let reply_message = self.repository.find_message_by_id(reply_id).await?;
            if let Some(msg) = reply_message {
                if msg.conversation_id != conversation_id {
                    return Err(MessagingError::Validation(
                        "Reply message must be in the same conversation".into()
                    ));
                }
            } else {
                return Err(MessagingError::NotFound("Reply message not found".into()));
            }
        }

        let message = self.repository.create_message(
            conversation_id,
            sender_id,
            content,
            message_type,
            reply_to_id,
        ).await?;

        Ok(MessageSentResponse {
            message_id: message.message_id,
            conversation_id: message.conversation_id,
            content: message.content,
            created_at: message.created_at,
        })
    }

    /// Edit a message.
    pub async fn edit_message(
        &self,
        user_id: Uuid,
        conversation_id: Uuid,
        message_id: Uuid,
        content: &str,
    ) -> Result<MessageResponse, MessagingError> {
        let message = self.repository.find_message_by_id(message_id).await?
            .ok_or(MessagingError::NotFound("Message not found".into()))?;

        // Verify ownership
        if message.sender_id != user_id {
            return Err(MessagingError::Forbidden("Can only edit your own messages".into()));
        }

        // Verify conversation
        if message.conversation_id != conversation_id {
            return Err(MessagingError::NotFound("Message not in this conversation".into()));
        }

        // Check if message is deleted
        if message.is_deleted {
            return Err(MessagingError::Validation("Cannot edit deleted message".into()));
        }

        let updated = self.repository.update_message(message_id, content).await?;

        Ok(MessageResponse {
            message_id: updated.message_id,
            conversation_id: updated.conversation_id,
            sender: SenderResponse {
                user_id: updated.sender_id,
                name: format!("User {}", updated.sender_id),
                avatar_url: None,
            },
            content: updated.content,
            message_type: updated.message_type,
            reply_to_id: updated.reply_to_id,
            is_edited: updated.is_edited,
            is_deleted: updated.is_deleted,
            created_at: updated.created_at,
            updated_at: updated.updated_at,
        })
    }

    /// Delete a message (soft delete).
    pub async fn delete_message(
        &self,
        user_id: Uuid,
        conversation_id: Uuid,
        message_id: Uuid,
    ) -> Result<(), MessagingError> {
        let message = self.repository.find_message_by_id(message_id).await?
            .ok_or(MessagingError::NotFound("Message not found".into()))?;

        // Verify ownership
        if message.sender_id != user_id {
            return Err(MessagingError::Forbidden("Can only delete your own messages".into()));
        }

        // Verify conversation
        if message.conversation_id != conversation_id {
            return Err(MessagingError::NotFound("Message not in this conversation".into()));
        }

        self.repository.soft_delete_message(message_id).await?;

        Ok(())
    }

    /// Mark messages as read.
    pub async fn mark_as_read(
        &self,
        user_id: Uuid,
        conversation_id: Uuid,
    ) -> Result<(), MessagingError> {
        let is_participant = self.repository.is_participant(user_id, conversation_id).await?;
        if !is_participant {
            return Err(MessagingError::Forbidden("Not a participant in this conversation".into()));
        }

        self.repository.update_last_read(user_id, conversation_id).await?;

        Ok(())
    }

    // =========================================================================
    // Search and Stats
    // =========================================================================

    /// Search messages.
    pub async fn search_messages(
        &self,
        user_id: Uuid,
        query: &str,
        conversation_id: Option<Uuid>,
        page: i32,
        per_page: i32,
    ) -> Result<PaginatedMessagesResponse, MessagingError> {
        // If conversation_id provided, verify participation
        if let Some(conv_id) = conversation_id {
            let is_participant = self.repository.is_participant(user_id, conv_id).await?;
            if !is_participant {
                return Err(MessagingError::Forbidden("Not a participant in this conversation".into()));
            }
        }

        let offset = (page - 1) * per_page;

        let messages = self.repository.search_messages(
            user_id,
            query,
            conversation_id,
            per_page,
            offset,
        ).await?;

        let total = self.repository.count_search_results(
            user_id,
            query,
            conversation_id,
        ).await?;

        let total_pages = ((total as f64) / (per_page as f64)).ceil() as i32;
        let has_more = page < total_pages;

        let items: Vec<MessageResponse> = messages.iter().map(|m| {
            MessageResponse {
                message_id: m.message_id,
                conversation_id: m.conversation_id,
                sender: SenderResponse {
                    user_id: m.sender_id,
                    name: format!("User {}", m.sender_id),
                    avatar_url: None,
                },
                content: m.content.clone(),
                message_type: m.message_type.clone(),
                reply_to_id: m.reply_to_id,
                is_edited: m.is_edited,
                is_deleted: m.is_deleted,
                created_at: m.created_at,
                updated_at: m.updated_at,
            }
        }).collect();

        Ok(PaginatedMessagesResponse {
            items,
            total,
            page,
            per_page,
            total_pages,
            has_more,
        })
    }

    /// Get unread counts for a user.
    pub async fn get_unread_counts(
        &self,
        user_id: Uuid,
    ) -> Result<UnreadCountResponse, MessagingError> {
        let counts = self.repository.get_all_unread_counts(user_id).await?;

        let total_unread: i64 = counts.iter().map(|c| c.unread_count).sum();

        let conversations: Vec<ConversationUnreadResponse> = counts.iter().map(|c| {
            ConversationUnreadResponse {
                conversation_id: c.conversation_id,
                unread_count: c.unread_count,
            }
        }).collect();

        Ok(UnreadCountResponse {
            total_unread,
            conversations,
        })
    }

    /// Leave a conversation.
    pub async fn leave_conversation(
        &self,
        user_id: Uuid,
        conversation_id: Uuid,
    ) -> Result<(), MessagingError> {
        let is_participant = self.repository.is_participant(user_id, conversation_id).await?;
        if !is_participant {
            return Err(MessagingError::Forbidden("Not a participant in this conversation".into()));
        }

        self.repository.leave_conversation(user_id, conversation_id).await?;

        Ok(())
    }

    /// Get or create a direct conversation with another user.
    pub async fn get_or_create_direct_conversation(
        &self,
        user_id: Uuid,
        other_user_id: Uuid,
    ) -> Result<ConversationResponse, MessagingError> {
        if user_id == other_user_id {
            return Err(MessagingError::CannotMessageSelf);
        }

        // Check if conversation already exists
        if let Some(existing) = self.repository.find_direct_conversation(user_id, other_user_id).await? {
            return self.get_conversation(user_id, existing.conversation_id).await;
        }

        // Create new direct conversation
        self.create_conversation(
            user_id,
            "direct",
            None,
            None,
            &[other_user_id],
        ).await
    }
}
