use lib::error_code::ErrorCode;
use num_derive::FromPrimitive;
use serde::*;
use strum_macros::EnumString;
use tokio_postgres::types::*;
#[derive(
    Debug, Clone, Copy, ToSql, FromSql, Serialize, Deserialize, FromPrimitive, PartialEq, EnumString,
)]
#[postgres(name = "enum_role")]
pub enum EnumRole {
    ///
    #[postgres(name = "guest")]
    Guest = 0,
    ///
    #[postgres(name = "user")]
    User = 1,
    ///
    #[postgres(name = "admin")]
    Admin = 2,
    ///
    #[postgres(name = "owner")]
    Owner = 3,
    ///
    #[postgres(name = "developer")]
    Developer = 4,
}
#[derive(
    Debug, Clone, Copy, ToSql, FromSql, Serialize, Deserialize, FromPrimitive, PartialEq, EnumString,
)]
#[postgres(name = "enum_recovery_question_category")]
pub enum EnumRecoveryQuestionCategory {
    ///
    #[postgres(name = "childhood")]
    Childhood = 0,
    ///
    #[postgres(name = "education")]
    Education = 1,
    ///
    #[postgres(name = "family")]
    Family = 2,
    ///
    #[postgres(name = "favorite")]
    Favorite = 3,
    ///
    #[postgres(name = "first")]
    First = 4,
    ///
    #[postgres(name = "personal")]
    Personal = 5,
    ///
    #[postgres(name = "pet")]
    Pet = 6,
    ///
    #[postgres(name = "work")]
    Work = 7,
    ///
    #[postgres(name = "historical")]
    Historical = 8,
}
#[derive(
    Debug, Clone, Copy, ToSql, FromSql, Serialize, Deserialize, FromPrimitive, PartialEq, EnumString,
)]
#[postgres(name = "enum_service")]
pub enum EnumService {
    ///
    #[postgres(name = "user")]
    User = 2,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorBadRequest {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInternalServerError {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorNotImplemented {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorNotFound {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorDatabaseError {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInvalidService {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorUserForbidden {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorUserNotFound {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorUserMustAgreeTos {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorUserMustAgreePrivacyPolicy {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInvalidEnumLevel {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorError {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInvalidArgument {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInvalidState {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInvalidSeq {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInvalidMethod {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorProtocolViolation {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorMalformedRequest {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorUnknownUser {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorBlockedUser {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInvalidPassword {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInvalidToken {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorTemporarilyUnavailable {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorUnexpectedException {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorBackPressureIncreased {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInvalidPublicId {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInvalidRange {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorBankAccountAlreadyExists {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInsufficientFunds {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorLogicalError {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorRestrictedUserPrivileges {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorIdenticalReplacement {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInvalidRecoveryQuestions {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInvalidRole {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorWrongRecoveryAnswers {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorMessageNotDelivered {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorNoReply {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorNullAttribute {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorConsentMissing {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorActiveSubscriptionRequired {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorUsernameAlreadyRegistered {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorRecoveryQuestionsNotSet {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorMustSubmitAllRecoveryQuestions {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInvalidRecoveryToken {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorRoutingError {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorUnauthorizedMessage {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorAuthError {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInternalError {}
#[derive(
    Debug, Clone, Copy, ToSql, FromSql, Serialize, Deserialize, FromPrimitive, PartialEq, EnumString,
)]
#[postgres(name = "enum_ErrorCode")]
pub enum EnumErrorCode {
    /// Custom Bad Request
    #[postgres(name = "BadRequest")]
    BadRequest = 100400,
    /// Custom Internal Server Error
    #[postgres(name = "InternalServerError")]
    InternalServerError = 100500,
    /// Custom Method not implemented
    #[postgres(name = "NotImplemented")]
    NotImplemented = 100501,
    /// Custom NotFoundResource
    #[postgres(name = "NotFound")]
    NotFound = 100404,
    /// Custom Database error
    #[postgres(name = "DatabaseError")]
    DatabaseError = 100601,
    /// Custom Invalid Service
    #[postgres(name = "InvalidService")]
    InvalidService = 100602,
    /// Custom Insufficient role for user
    #[postgres(name = "UserForbidden")]
    UserForbidden = 101403,
    /// Custom User not found
    #[postgres(name = "UserNotFound")]
    UserNotFound = 101404,
    /// Custom Must agree to the terms of service
    #[postgres(name = "UserMustAgreeTos")]
    UserMustAgreeTos = 101601,
    /// Custom Must agree to the privacy policy
    #[postgres(name = "UserMustAgreePrivacyPolicy")]
    UserMustAgreePrivacyPolicy = 101602,
    /// SQL 22P02 InvalidEnumLevel
    #[postgres(name = "InvalidEnumLevel")]
    InvalidEnumLevel = 3484946,
    /// SQL R0000 Error
    #[postgres(name = "Error")]
    Error = 4349632,
    /// SQL R0001 InvalidArgument
    #[postgres(name = "InvalidArgument")]
    InvalidArgument = 45349633,
    /// SQL R0002 InvalidState
    #[postgres(name = "InvalidState")]
    InvalidState = 45349634,
    /// SQL R0003 InvalidSeq
    #[postgres(name = "InvalidSeq")]
    InvalidSeq = 45349635,
    /// SQL R0004 InvalidMethod
    #[postgres(name = "InvalidMethod")]
    InvalidMethod = 45349636,
    /// SQL R0005 ProtocolViolation
    #[postgres(name = "ProtocolViolation")]
    ProtocolViolation = 45349637,
    /// SQL R0006 MalformedRequest
    #[postgres(name = "MalformedRequest")]
    MalformedRequest = 45349638,
    /// SQL R0007 UnknownUser
    #[postgres(name = "UnknownUser")]
    UnknownUser = 45349639,
    /// SQL R0008 BlockedUser
    #[postgres(name = "BlockedUser")]
    BlockedUser = 45349640,
    /// SQL R0009 InvalidPassword
    #[postgres(name = "InvalidPassword")]
    InvalidPassword = 45349641,
    /// SQL R000A InvalidToken
    #[postgres(name = "InvalidToken")]
    InvalidToken = 45349642,
    /// SQL R000B TemporarilyUnavailable
    #[postgres(name = "TemporarilyUnavailable")]
    TemporarilyUnavailable = 45349643,
    /// SQL R000C UnexpectedException
    #[postgres(name = "UnexpectedException")]
    UnexpectedException = 45349644,
    /// SQL R000D BackPressureIncreased
    #[postgres(name = "BackPressureIncreased")]
    BackPressureIncreased = 45349645,
    /// SQL R000E InvalidPublicId
    #[postgres(name = "InvalidPublicId")]
    InvalidPublicId = 45349646,
    /// SQL R000F InvalidRange
    #[postgres(name = "InvalidRange")]
    InvalidRange = 45349647,
    /// SQL R000G BankAccountAlreadyExists
    #[postgres(name = "BankAccountAlreadyExists")]
    BankAccountAlreadyExists = 45349648,
    /// SQL R000H InsufficientFunds
    #[postgres(name = "InsufficientFunds")]
    InsufficientFunds = 45349649,
    /// SQL R000M LogicalError
    #[postgres(name = "LogicalError")]
    LogicalError = 45349654,
    /// SQL R000N RestrictedUserPrivileges
    #[postgres(name = "RestrictedUserPrivileges")]
    RestrictedUserPrivileges = 45349655,
    /// SQL R000O IdenticalReplacement
    #[postgres(name = "IdenticalReplacement")]
    IdenticalReplacement = 45349656,
    /// SQL R000R InvalidRecoveryQuestions
    #[postgres(name = "InvalidRecoveryQuestions")]
    InvalidRecoveryQuestions = 45349659,
    /// SQL R000S InvalidRole
    #[postgres(name = "InvalidRole")]
    InvalidRole = 45349660,
    /// SQL R000T WrongRecoveryAnswers
    #[postgres(name = "WrongRecoveryAnswers")]
    WrongRecoveryAnswers = 45349661,
    /// SQL R000U MessageNotDelivered
    #[postgres(name = "MessageNotDelivered")]
    MessageNotDelivered = 45349662,
    /// SQL R000V NoReply
    #[postgres(name = "NoReply")]
    NoReply = 45349663,
    /// SQL R000W NullAttribute
    #[postgres(name = "NullAttribute")]
    NullAttribute = 45349664,
    /// SQL R000X ConsentMissing
    #[postgres(name = "ConsentMissing")]
    ConsentMissing = 45349665,
    /// SQL R000Y ActiveSubscriptionRequired
    #[postgres(name = "ActiveSubscriptionRequired")]
    ActiveSubscriptionRequired = 45349666,
    /// SQL R000Z UsernameAlreadyRegistered
    #[postgres(name = "UsernameAlreadyRegistered")]
    UsernameAlreadyRegistered = 45349667,
    /// SQL R0010 RecoveryQuestionsNotSet
    #[postgres(name = "RecoveryQuestionsNotSet")]
    RecoveryQuestionsNotSet = 45349668,
    /// SQL R0011 MustSubmitAllRecoveryQuestions
    #[postgres(name = "MustSubmitAllRecoveryQuestions")]
    MustSubmitAllRecoveryQuestions = 45349669,
    /// SQL R0012 InvalidRecoveryToken
    #[postgres(name = "InvalidRecoveryToken")]
    InvalidRecoveryToken = 45349670,
    /// SQL R0018 RoutingError
    #[postgres(name = "RoutingError")]
    RoutingError = 45349676,
    /// SQL R0019 UnauthorizedMessage
    #[postgres(name = "UnauthorizedMessage")]
    UnauthorizedMessage = 45349677,
    /// SQL R001B AuthError
    #[postgres(name = "AuthError")]
    AuthError = 45349679,
    /// SQL R001G InternalError
    #[postgres(name = "InternalError")]
    InternalError = 45349684,
}

impl Into<ErrorCode> for EnumErrorCode {
    fn into(self) -> ErrorCode {
        ErrorCode::new(self as _)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AddCrmLeadRequest {
    pub email: String,
    pub username: String,
    pub title: String,
    pub message: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AddCrmLeadResponse {}
