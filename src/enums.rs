use serde::Deserialize;

#[derive(Deserialize)]
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

#[derive(Deserialize, PartialEq)]
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
pub enum PaymentProcessor {
    Stripe,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PriceType {
    OneTime,
    Recurring,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RecurringInterval {
    Month,
    Year,
}
