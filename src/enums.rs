use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AmountType {
    Fixed,
    Custom,
    Free,
    MeteredUnit,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BenefitType {
    Custom,
    Discord,
    GithubRepository,
    Downloadables,
    LicenseKeys,
    MeterCredit,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BillingAddressField {
    Required,
    Optional,
    Disabled,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutSessionsSorting {
    CreatedAt,
    #[serde(rename = "-created_at")]
    CreatedAtDesc,
    ExpiresAt,
    #[serde(rename = "-expires_at")]
    ExpiresAtDesc,
    Status,
    #[serde(rename = "-status")]
    StatusDesc,
}

#[derive(Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CheckoutSessionStatus {
    Open,
    Expired,
    Confirmed,
    Succeeded,
    Failed,
}

impl CheckoutSessionStatus {
    pub fn is_succeeded(&self) -> bool {
        *self == Self::Succeeded
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CustomerCancellationReason {
    CustomerService,
    LowQuality,
    MissingFeatures,
    SwitchedService,
    TooComplex,
    TooExpensive,
    Unused,
    Other,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CustomFieldType {
    Text,
    Number,
    Date,
    Checkbox,
    Select,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DiscountDuration {
    Once,
    Forever,
    Repeating,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DiscountType {
    Fixed,
    Percentage,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MeterAggregationFunc {
    Count,
    Sum,
    Max,
    Min,
    Avg,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MeterFilterConjunction {
    And,
    Or,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MeterFilterOperator {
    Eq,
    Ne,
    Gt,
    Gte,
    Lt,
    Lte,
    Like,
    NotLike,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PaymentProcessor {
    Stripe,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PriceType {
    OneTime,
    Recurring,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProductsSorting {
    CreatedAt,
    #[serde(rename = "-created_at")]
    CreatedAtDesc,
    ExpiresAt,
    #[serde(rename = "-expires_at")]
    ExpiresAtDesc,
    Status,
    #[serde(rename = "-status")]
    StatusDesc,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ProrationBehavior {
    Invoice,
    Prorate,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum RecurringInterval {
    Month,
    Year,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionStatus {
    Incomplete,
    IncompleteExpired,
    Trialing,
    Active,
    PastDue,
    Cancelled,
    Unpaid,
}
