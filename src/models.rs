use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

use crate::enums::*;

#[derive(Deserialize)]
pub struct AttachedCustomField {
    /// ID of the custom field.
    pub custom_field_id: Uuid,
    /// Schema for a custom field of type text.
    pub custom_field: CustomField,
    /// Order of the custom field in the resource.
    pub order: usize,
    /// Whether the value is required for this custom field.
    pub required: bool,
}

#[derive(Deserialize, Serialize)]
pub struct AttachedCustomFieldParams {
    /// ID of the custom field.
    pub custom_field_id: Uuid,
    /// Whether the value is required for this custom field.
    pub required: bool,
}

#[derive(Deserialize)]
pub struct Benefit {
    /// The ID of the benefit.
    pub id: Uuid,
    /// Creation timestamp of the object.
    pub created_at: DateTime<Utc>,
    /// Last modification timestamp of the object.
    pub modified_at: Option<DateTime<Utc>>,
    /// The type of the benefit.
    pub r#type: BenefitType,
    /// The description of the benefit.
    pub description: String,
    /// Whether the benefit is selectable when creating a product.
    pub selectable: bool,
    /// Whether the benefit is deletable.
    pub deletable: bool,
    /// The ID of the organization owning the benefit.
    pub organization_id: Uuid,
}

#[derive(Deserialize)]
pub struct BillingAddressFields {
    pub country: BillingAddressField,
    pub state: BillingAddressField,
    pub city: BillingAddressField,
    pub postal_code: BillingAddressField,
    pub line1: BillingAddressField,
    pub line2: BillingAddressField,
}

#[derive(Deserialize)]
pub struct CheckoutProduct {
    /// Creation timestamp of the object.
    pub created_at: DateTime<Utc>,
    /// Last modification timestamp of the object.
    pub modified_at: Option<DateTime<Utc>>,
    /// The ID of the product.
    pub id: Uuid,
    /// The name of the product.
    pub name: String,
    /// The description of the product.
    pub description: Option<String>,
    /// The recurring interval of the product. If `None`, the product is a one-time purchase.
    pub recurring_interval: Option<RecurringInterval>,
    /// Whether the product is a subscription.
    pub is_recurring: bool,
    /// Whether the product is archived and no longer available.
    pub is_archived: bool,
    /// The ID of the organization owning the product.
    pub organization_id: Uuid,
    /// List of prices for this product.
    pub prices: Vec<Price>,
    /// List of benefits granted by the product.
    pub benefits: Vec<Benefit>,
    /// List of medias associated to the product.
    pub medias: Vec<Media>,
}

#[derive(Deserialize)]
pub struct CheckoutSession {
    /// Creation timestamp of the object.
    pub created_at: DateTime<Utc>,
    /// Last modification timestamp of the object.
    pub modified_at: Option<DateTime<Utc>>,
    /// The ID of the object.
    pub id: Uuid,
    /// Payment processor used.
    pub payment_processor: PaymentProcessor,
    /// Status of the checkout session.
    pub status: CheckoutSessionStatus,
    /// Client secret used to update and complete the checkout session from the client.
    pub client_secret: String,
    /// URL where the customer can access the checkout session.
    pub url: Url,
    /// Expiration date and time of the checkout session.
    pub expires_at: DateTime<Utc>,
    // URL where the customer will be redirected after a successful payment.
    pub success_url: Url,
    /// When checkout is embedded, represents the Origin of the page embedding the checkout. Used as a security measure to send messages only to the embedding page.
    pub embed_origin: Option<String>,
    /// Amount in cents, before discounts and taxes.
    pub amount: u32,
    /// Discount amount in cents.
    pub discount_amount: u32,
    /// Amount in cents, after discounts but before taxes.
    pub net_amount: u32,
    /// Sales tax amount in cents. If `null`, it means there is no enough information yet to calculate it.
    pub tax_amount: Option<u32>,
    /// Amount in cents, after discounts and taxes.
    pub total_amount: u32,
    /// Currency code of the checkout session.
    pub currency: String,
    /// ID of the product to checkout.
    pub product_id: Uuid,
    /// ID of the product price to checkout.
    pub product_price_id: Uuid,
    /// ID of the discount applied to the checkout.
    pub discount_id: Option<Uuid>,
    /// Whether to allow the customer to apply discount codes. If you apply a discount through `discount_id`, it'll still be applied, but the customer won't be able to change it.
    pub allow_discount_codes: bool,
    /// Whether to require the customer to fill their full billing address, instead of just the country. Customers in the US will always be required to fill their full address, regardless of this setting. If you preset the billing address, this setting will be automatically set to `true`.
    pub require_billing_address: bool,
    /// Whether the discount is applicable to the checkout. Typically, free and custom prices are not discountable.
    pub is_discount_applicable: bool,
    /// Whether the product price is free, regardless of discounts.
    pub is_free_product_price: bool,
    /// Whether the checkout requires payment, e.g. in case of free products or discounts that cover the total amount.
    pub is_payment_required: bool,
    /// Whether the checkout requires setting up a payment method, regardless of the amount, e.g. subscriptions that have first free cycles.
    pub is_payment_setup_required: bool,
    /// Whether the checkout requires a payment form, whether because of a payment or payment method setup.
    pub is_payment_form_required: bool,
    pub customer_id: Option<Uuid>,
    /// Whether the customer is a business or an individual. If `true`, the customer will be required to fill their full billing address and billing name.
    pub is_business_customer: bool,
    /// Name of the customer.
    pub customer_name: Option<String>,
    /// Email address of the customer.
    pub customer_email: Option<String>,
    pub customer_ip_address: Option<String>,
    pub customer_billing_name: Option<String>,
    /// Billing address of the customer.
    pub customer_billing_address: Option<CustomerBillingAddress>,
    pub customer_tax_id: Option<String>,
    pub payment_processor_metadata: HashMap<String, String>,
    /// Determine which billing address fields should be disabled, optional or required in the checkout form.
    pub billing_address_fields: BillingAddressFields,
    pub metadata: HashMap<String, String>,
    pub external_customer_id: Option<String>,
    /// List of products available to select.
    pub products: Vec<CheckoutProduct>,
    /// Product selected to checkout.
    pub product: CheckoutProduct,
    /// Price of the selected product.
    pub product_price: Price,
    /// Schema for a percentage discount that is applied on every invoice for a certain number of months.
    pub discount: Option<Discount>,
    pub subscription_id: Option<Uuid>,
    pub attached_custom_fields: Vec<AttachedCustomField>,
    pub customer_metadata: HashMap<String, String>,
    pub custom_field_data: HashMap<String, Option<String>>,
}

#[derive(Default, Deserialize, Serialize)]
pub struct CheckoutSessionParams {
    /// List of product IDs available to select at that checkout. The first one will be selected by default.
    pub products: Vec<Uuid>,
    /// Key-value object allowing you to store additional information.
    pub metadata: HashMap<String, String>,
    /// Key-value object storing custom field values.
    pub custom_field_data: HashMap<String, String>,
    /// ID of the discount to apply to the checkout.
    pub discount_id: Option<Uuid>,
    /// Whether to allow the customer to apply discount codes. If you apply a discount through `discount_id`, it'll still be applied, but the customer won't be able to change it.
    pub allow_discount_codes: bool,
    /// Whether to require the customer to fill their full billing address, instead of just the country. Customers in the US will always be required to fill their full address, regardless of this setting. If you preset the billing address, this setting will be automatically set to `true`.
    pub require_billing_address: bool,
    /// Amount in cents, before discounts and taxes. Only useful for custom prices, it'll be ignored for fixed and free prices.
    pub amount: Option<u32>,
    /// ID of an existing customer in the organization. The customer data will be pre-filled in the checkout form. The resulting order will be linked to this customer.
    pub customer_id: Option<Uuid>,
    /// Whether the customer is a business or an individual. If `true`, the customer will be required to fill their full billing address and billing name.
    pub is_business_customer: bool,
    /// ID of the customer in your system. If a matching customer exists on Polar, the resulting order will be linked to this customer. Otherwise, a new customer will be created with this external ID set.
    pub external_customer_id: Option<String>,
    /// Name of the customer.
    pub customer_name: Option<String>,
    /// Email address of the customer.
    pub customer_email: Option<String>,
    pub customer_ip_address: Option<String>,
    pub customer_billing_name: Option<String>,
    /// Billing address of the customer.
    pub customer_billing_address: Option<CustomerBillingAddressParams>,
    pub customer_tax_id: Option<String>,
    /// Key-value object allowing you to store additional information that'll be copied to the created customer.
    pub customer_metadata: HashMap<String, String>,
    /// ID of a subscription to upgrade. It must be on a free pricing. If checkout is successful, metadata set on this checkout will be copied to the subscription, and existing keys will be overwritten.
    pub subscription_id: Option<Uuid>,
    ///URL where the customer will be redirected after a successful payment.You can add the `checkout_id={CHECKOUT_ID}` query parameter to retrieve the checkout session id.
    pub success_url: Option<Url>,
    /// If you plan to embed the checkout session, set this to the Origin of the embedding page. It'll allow the Polar iframe to communicate with the parent page.
    pub embed_origin: Option<String>,
}

#[derive(Deserialize)]
pub struct CustomField {
    /// Creation timestamp of the object.
    pub created_at: DateTime<Utc>,
    /// Last modification timestamp of the object.
    pub modified_at: Option<DateTime<Utc>>,
    /// The ID of the object.
    pub id: Uuid,
    pub metadata: HashMap<String, String>,
    pub r#type: CustomFieldType,
    /// Identifier of the custom field. It'll be used as key when storing the value.
    pub slug: String,
    /// Name of the custom field.
    pub name: String,
    /// The ID of the organization owning the custom field.
    pub organization_id: Uuid,
    pub properties: CustomFieldProperties,
}

#[derive(Deserialize)]
pub struct CustomFieldOption {
    /// Minimum length: `1`
    pub value: String,
    /// Minimum length: `1`
    pub label: String,
}

#[derive(Deserialize)]
pub struct CustomFieldProperties {
    /// Minimum length: `1`
    pub form_label: String,
    /// Minimum length: `1`
    pub form_help_text: String,
    /// Minimum length: `1`
    pub form_placeholder: String,
    pub textarea: Option<bool>,
    /// Required range: `x >= 0`
    pub min_length: Option<usize>,
    /// Required range: `x >= 0`
    pub max_length: Option<usize>,
    pub ge: Option<usize>,
    pub le: Option<usize>,
    pub options: Option<Vec<CustomFieldOption>>,
}

#[derive(Deserialize)]
pub struct Customer {
    /// The ID of the customer.
    pub id: Uuid,
    /// Creation timestamp of the object.
    pub created_at: DateTime<Utc>,
    /// Last modification timestamp of the object.
    pub modified_at: Option<DateTime<Utc>>,
    pub metadata: HashMap<String, String>,
    /// The ID of the customer in your system. This must be unique within the organization. Once set, it can't be updated.
    pub external_id: Option<String>,
    /// The email address of the customer. This must be unique within the organization.
    pub email: String,
    /// Whether the customer email address is verified. The address is automatically verified when the customer accesses the customer portal using their email address.
    pub email_verified: bool,
    /// The name of the customer.
    pub name: Option<String>,
    pub billing_address: Option<CustomerBillingAddress>,
    /// Required array length: 2 elements
    pub tax_id: Option<Vec<String>>,
    /// The ID of the organization owning the customer.
    pub organization_id: Uuid,
    /// Timestamp for when the customer was soft deleted.
    pub deleted_at: Option<DateTime<Utc>>,
    pub avatar_url: String,
}

#[derive(Deserialize, Serialize)]
pub struct CustomerBillingAddress {
    /// Examples: `"US"` `"SE"` `"FR"`
    country: String,
    line1: Option<String>,
    line2: Option<String>,
    postal_code: Option<String>,
    city: Option<String>,
    state: Option<String>,
}

pub type CustomerBillingAddressParams = CustomerBillingAddress;

#[derive(Deserialize)]
pub struct Discount {
    pub duration: DiscountDuration,
    pub duration_in_months: Option<usize>,
    pub r#type: DiscountType,
    pub amount: Option<u32>,
    pub currency: Option<String>,
    pub basis_points: Option<usize>,
    /// The ID of the object.
    pub id: Uuid,
    pub name: String,
    pub code: Option<String>,
}

#[derive(Default, Serialize)]
pub struct ListCheckoutSessionsParams {
    /// Filter by organization ID.
    pub organization_id: Option<Vec<Uuid>>,
    /// Filter by product ID.
    pub product_id: Option<Vec<Uuid>>,
    /// Filter by customer ID.
    pub customer_id: Option<Vec<Uuid>>,
    /// Filter by checkout session status.
    pub status: Option<CheckoutSessionStatus>,
    /// Filter by customer email.
    pub query: Option<String>,
    /// Page number, defaults to 1.
    ///
    /// Required range: `x > 0`
    pub page: Option<usize>,
    /// Size of a page, defaults to 10. Maximum is 100.
    ///
    /// Required range: `x > 0`
    pub limit: Option<u8>,
    /// Sorting criterion. Several criteria can be used simultaneously and will be applied in order. Add a minus sign - before the criteria name to sort by descending order.
    pub sorting: Option<Vec<CheckoutSessionsSorting>>,
}

#[derive(Default, Serialize)]
pub struct ListMetersParams {
    /// Filter by organization ID.
    pub organization_id: Option<Vec<Uuid>>,
    /// Filter by name.
    pub query: Option<String>,
    /// Page number, defaults to 1.
    ///
    /// Required range: `x > 0`
    pub page: Option<usize>,
    /// Size of a page, defaults to 10. Maximum is 100.
    ///
    /// Required range: `x > 0`
    pub limit: Option<u8>,
    /// Sorting criterion. Several criteria can be used simultaneously and will be applied in order. Add a minus sign - before the criteria name to sort by descending order.
    pub sorting: Option<Vec<MetersSorting>>,
    /// Filter by metadata key-value pairs.
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Default, Serialize)]
pub struct ListProductsParams {
    /// Filter by product ID.
    pub id: Option<Vec<Uuid>>,
    /// Filter by organization ID.
    pub organization_id: Option<Vec<Uuid>>,
    /// Filter by product name.
    pub query: Option<String>,
    /// Filter on archived products.
    pub is_archived: Option<bool>,
    /// Filter on recurring products. If `true`, only subscriptions tiers are returned. If `false`, only one-time purchase products are returned.
    pub is_recurring: Option<bool>,
    /// Filter products granting specific benefit.
    pub benefit_id: Option<Vec<Uuid>>,
    /// Page number, defaults to 1.
    ///
    /// Required range: `x > 0`
    pub page: Option<usize>,
    /// Size of a page, defaults to 10. Maximum is 100.
    ///
    /// Required range: `x > 0`
    pub limit: Option<u8>,
    /// Sorting criterion. Several criteria can be used simultaneously and will be applied in order. Add a minus sign - before the criteria name to sort by descending order.
    pub sorting: Option<Vec<ProductsSorting>>,
    /// Filter by metadata key-value pairs.
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Default, Serialize)]
pub struct ListSubscriptionsParams {
    /// Filter by organization ID.
    pub organization_id: Option<Vec<Uuid>>,
    /// Filter by product ID.
    pub product_id: Option<Vec<Uuid>>,
    /// Filter by customer ID.
    pub customer_id: Option<Vec<Uuid>>,
    /// Filter by customer external ID.
    pub external_customer_id: Option<Vec<String>>,
    /// Filter by discount ID.
    pub discount_id: Option<Vec<Uuid>>,
    /// Filter by active or inactive subscription.
    pub active: Option<bool>,
    /// Page number, defaults to 1.
    ///
    /// Required range: `x > 0`
    pub page: Option<usize>,
    /// Size of a page, defaults to 10. Maximum is 100.
    ///
    /// Required range: `x > 0`
    pub limit: Option<u8>,
    /// Sorting criterion. Several criteria can be used simultaneously and will be applied in order. Add a minus sign - before the criteria name to sort by descending order.
    pub sorting: Option<Vec<SubscriptionsSorting>>,
    /// Filter by metadata key-value pairs.
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Deserialize)]
pub struct Media {
    /// The ID of the object.
    pub id: Uuid,
    pub organization_id: Uuid,
    pub name: String,
    pub path: String,
    pub mime_type: String,
    pub size: u64,
    pub storage_version: Option<String>,
    pub checksum_etag: Option<String>,
    pub checksum_sha256_base64: Option<String>,
    pub checksum_sha256_hex: Option<String>,
    pub last_modified_at: Option<DateTime<Utc>>,
    pub version: Option<String>,
    pub service: String,
    pub is_uploaded: bool,
    pub created_at: DateTime<Utc>,
    pub size_readable: String,
    pub public_url: Url,
}

#[derive(Deserialize)]
pub struct Meter {
    pub metadata: HashMap<String, String>,
    /// Creation timestamp of the object.
    pub created_at: DateTime<Utc>,
    /// Last modification timestamp of the object.
    pub modified_at: Option<DateTime<Utc>>,
    /// The ID of the object.
    pub id: Uuid,
    /// The name of the meter. Will be shown on customer's invoices and usage.
    pub name: String,
    /// The filter to apply on events that'll be used to calculate the meter.
    pub filter: MeterFilter,
    /// The aggregation to apply on the filtered events to calculate the meter.
    pub aggregation: MeterAggregation,
    /// The ID of the organization owning the meter.
    pub organization_id: Uuid,
}

#[derive(Deserialize, Serialize)]
pub struct MeterAggregation {
    pub func: MeterAggregationFunc,
    pub property: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct MeterFilter {
    pub conjunction: MeterFilterConjunction,
    pub clauses: Vec<MeterFilterClause>,
}

#[derive(Deserialize, Serialize)]
pub struct MeterFilterClause {
    pub property: Option<String>,
    pub operator: Option<MeterFilterOperator>,
    pub value: Option<String>,
    pub conjunction: Option<MeterFilterConjunction>,
    pub clauses: Option<Vec<MeterFilterClause>>,
}

#[derive(Deserialize, Serialize)]
pub struct MeterParams {
    /// The name of the meter. Will be shown on customer's invoices and usage.
    ///
    /// Minimum length: `3`
    pub name: String,
    /// The filter to apply on events that'll be used to calculate the meter.
    pub filter: MeterFilter,
    /// The aggregation to apply on the filtered events to calculate the meter.
    pub aggregation: MeterAggregation,
    /// Key-value object allowing you to store additional information.
    pub metadata: HashMap<String, String>,
    /// The ID of the organization owning the meter. **Required unless you use an organization token**.
    pub organization_id: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct Page<T> {
    pub items: Vec<T>,
    pub pagination: Pagination,
}

#[derive(Deserialize)]
pub struct Pagination {
    pub total_count: usize,
    pub max_page: usize,
}

#[derive(Deserialize)]
pub struct Price {
    /// Creation timestamp of the object.
    pub created_at: DateTime<Utc>,
    /// Last modification timestamp of the object.
    pub modified_at: Option<DateTime<Utc>>,
    /// The ID of the price.
    pub id: Uuid,
    pub amount_type: AmountType,
    /// Whether the price is archived and no longer available.
    pub is_archived: bool,
    /// The ID of the product owning the price.
    pub product_id: Uuid,
    pub r#type: PriceType,
    /// The currency. Not required for `amount_type: Free`.
    pub price_currency: Option<String>,
    /// The price in cents.  Only for `amount_type: Fixed`.
    pub price_amount: Option<u32>,
    /// The minimum amount the customer can pay. Only for `amount_type: Custom`.
    pub minimum_amount: Option<u32>,
    /// The maximum amount the customer can pay. Only for `amount_type: Custom`.
    pub maximum_amount: Option<u32>,
    /// The initial amount shown to the customer. Only for `amount_type: Custom`.
    pub preset_amount: Option<u32>,
    /// The price per unit in cents. Only for `amount_type: MeteredUnit`.
    pub unit_amount: Option<String>,
    /// The maximum amount in cents that can be charged, regardless of the number of units consumed. Only for `amount_type: MeteredUnit`.
    pub cap_amount: Option<u32>,
    /// The ID of the meter associated to the price. Only for `amount_type: MeteredUnit`.
    pub meter_id: Option<Uuid>,
    /// The meter associated to the price. Only for `amount_type: MeteredUnit`.
    pub meter: Option<PriceMeter>,
}

#[derive(Deserialize)]
pub struct PriceMeter {
    /// The ID of the object.
    pub id: Uuid,
    /// The name of the meter.
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct PriceParams {
    pub amount_type: AmountType,
    /// The currency. Not required for `amount_type: Free`.
    pub price_currency: Option<String>,
    /// The price in cents.  Only for `amount_type: Fixed`.
    pub price_amount: Option<u32>,
    /// The minimum amount the customer can pay. Only for `amount_type: Custom`.
    pub minimum_amount: Option<u32>,
    /// The maximum amount the customer can pay. Only for `amount_type: Custom`.
    pub maximum_amount: Option<u32>,
    /// The initial amount shown to the customer. Only for `amount_type: Custom`.
    pub preset_amount: Option<u32>,
    /// The ID of the meter associated to the price. Only for `amount_type: MeteredUnit`.
    pub meter_id: Option<Uuid>,
    /// The price per unit in cents. Only for `amount_type: MeteredUnit`.
    pub unit_amount: Option<String>,
    /// The maximum amount in cents that can be charged, regardless of the number of units consumed. Only for `amount_type: MeteredUnit`.
    pub cap_amount: Option<u32>,
}

#[derive(Deserialize)]
pub struct Product {
    /// Creation timestamp of the object.
    pub created_at: DateTime<Utc>,
    /// Last modification timestamp of the object.
    pub modified_at: Option<DateTime<Utc>>,
    /// The ID of the product.
    pub id: Uuid,
    /// The name of the product.
    pub name: String,
    /// The description of the product.
    pub description: Option<String>,
    /// The recurring interval of the product. If `None`, the product is a one-time purchase.
    pub recurring_interval: Option<RecurringInterval>,
    /// Whether the product is a subscription.
    pub is_recurring: bool,
    /// Whether the product is archived and no longer available.
    pub is_archived: bool,
    /// The ID of the organization owning the product.
    pub organization_id: Uuid,
    pub metadata: HashMap<String, String>,
    /// List of prices for this product.
    pub prices: Vec<Price>,
    /// List of benefits granted by the product.
    pub benefits: Vec<Benefit>,
    /// List of medias associated to the product.
    pub medias: Vec<Media>,
    /// List of custom fields attached to the product.
    pub attached_custom_fields: Vec<AttachedCustomField>,
}

#[derive(Default, Deserialize, Serialize)]
pub struct ProductParams {
    /// The name of the product.
    ///
    /// Minimum length: `3`
    pub name: String,
    /// The recurring interval of the product. If `None`, the product is a one-time purchase
    pub recurring_interval: Option<RecurringInterval>,
    /// List of available prices for this product. It should contain at most one static price (fixed, custom or free), and any number of metered prices. Metered prices are not supported on one-time purchase products.
    pub prices: Vec<PriceParams>,
    /// Key-value object allowing you to store additional information.
    pub metadata: HashMap<String, String>,
    /// The description of the product.
    pub description: Option<String>,
    /// List of file IDs. Each one must be on the same organization as the product, of type `product_media` and correctly uploaded.
    pub medias: Option<Vec<Uuid>>,
    /// List of custom fields to attach.
    pub attached_custom_fields: Vec<AttachedCustomFieldParams>,
    /// The ID of the organization owning the product. **Required unless you use an organization token**.
    pub organization_id: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct Subscription {
    /// Creation timestamp of the object.
    pub created_at: DateTime<Utc>,
    /// Last modification timestamp of the object.
    pub modified_at: Option<DateTime<Utc>>,
    /// The ID of the object.
    pub id: Uuid,
    /// The amount of the subscription.
    pub amount: u32,
    /// The currency of the subscription.
    pub currency: String,
    /// The interval at which the subscription recurs.
    pub recurring_interval: RecurringInterval,
    /// The status of the subscription.
    pub status: SubscriptionStatus,
    /// The start timestamp of the current billing period.
    pub current_period_start: DateTime<Utc>,
    /// The end timestamp of the current billing period.
    pub current_period_end: Option<DateTime<Utc>>,
    /// Whether the subscription will be canceled at the end of the current period.
    pub cancel_at_period_end: bool,
    /// The timestamp when the subscription was canceled. The subscription might still be active if `cancel_at_period_end` is `true`.
    pub canceled_at: Option<DateTime<Utc>>,
    /// The timestamp when the subscription started.
    pub started_at: Option<DateTime<Utc>>,
    /// The timestamp when the subscription will end.
    pub ends_at: Option<DateTime<Utc>>,
    /// The timestamp when the subscription ended.
    pub ended_at: Option<DateTime<Utc>>,
    /// The ID of the subscribed customer.
    pub customer_id: Uuid,
    /// The ID of the subscribed product.
    pub product_id: Uuid,
    /// The ID of the applied discount, if any.
    pub discount_id: Option<Uuid>,
    pub checkout_id: Option<Uuid>,
    pub customer_cancellation_reason: Option<CustomerCancellationReason>,
    pub customer_cancellation_comment: Option<String>,
    pub metadata: HashMap<String, String>,
    pub customer: Customer,
    /// A product.
    pub product: Product,
    pub discount: Option<Discount>,
    /// List of enabled prices for the subscription.
    pub prices: Vec<Price>,
    /// List of meters associated with the subscription.
    pub meters: Vec<SubscriptionMeter>,
    // Key-value object storing custom field values.
    pub custom_field_data: HashMap<String, String>,
}

#[derive(Deserialize)]
pub struct SubscriptionMeter {
    /// Creation timestamp of the object.
    pub created_at: DateTime<Utc>,
    /// Last modification timestamp of the object.
    pub modified_at: Option<DateTime<Utc>>,
    /// The ID of the object.
    pub id: Uuid,
    /// The number of consumed units so far in this billing period.
    pub consumed_units: usize,
    /// The number of credited units so far in this billing period.
    pub credited_units: usize,
    /// The amount due in cents so far in this billing period.
    pub amount: u32,
    /// The ID of the meter.
    pub meter_id: Uuid,
    /// The meter associated with this subscription.
    pub meter: Meter,
}

#[derive(Default, Deserialize, Serialize)]
pub struct SubscriptionParams {
    /// Update subscription to another product.
    pub product_id: Option<Uuid>,
    /// Determine how to handle the proration billing. If not provided, will use the default organization setting.
    pub proration_behavior: Option<ProrationBehavior>,
    /// Update the subscription to apply a new discount. If set to `None`, the discount will be removed. The change will be applied on the next billing cycle.
    pub discount_id: Option<Uuid>,
    /// Cancel an active subscription once the current period ends.
    ///
    /// Or uncancel a subscription currently set to be revoked at period end.
    pub cancel_at_period_end: Option<bool>,
    /// Customer reason for cancellation. Helpful to monitor reasons behind churn for future improvements.
    ///
    /// Only set this in case your own service is requesting the reason from the customer. Or you know based on direct conversations, i.e support, with the customer.
    pub customer_cancellation_reason: Option<CustomerCancellationReason>,
    /// Customer feedback and why they decided to cancel.
    pub customer_cancellation_comment: Option<String>,
    /// Cancel and revoke an active subscription immediately
    pub revoke: Option<bool>,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateMeterParams {
    /// Key-value object allowing you to store additional information.
    pub metadata: HashMap<String, String>,
    /// The name of the meter. Will be shown on customer's invoices and usage.
    ///
    /// Minimum length: `3`
    pub name: Option<String>,
    /// The filter to apply on events that'll be used to calculate the meter.
    pub filter: Option<MeterFilter>,
    /// The aggregation to apply on the filtered events to calculate the meter.
    pub aggregation: Option<MeterAggregation>,
}

#[derive(Deserialize, Serialize)]
pub struct UpdatePriceParams {
    /// If you want to keep the existing price.
    pub id: Option<Uuid>,
    pub amount_type: Option<AmountType>,
    /// The currency. Not required for `amount_type: Free`.
    pub price_currency: Option<String>,
    /// The price in cents.  Only for `amount_type: Fixed`.
    pub price_amount: Option<u32>,
    /// The minimum amount the customer can pay. Only for `amount_type: Custom`.
    pub minimum_amount: Option<u32>,
    /// The maximum amount the customer can pay. Only for `amount_type: Custom`.
    pub maximum_amount: Option<u32>,
    /// The initial amount shown to the customer. Only for `amount_type: Custom`.
    pub preset_amount: Option<u32>,
    /// The ID of the meter associated to the price. Only for `amount_type: MeteredUnit`.
    pub meter_id: Option<Uuid>,
    /// The price per unit in cents. Only for `amount_type: MeteredUnit`.
    pub unit_amount: Option<String>,
    /// The maximum amount in cents that can be charged, regardless of the number of units consumed. Only for `amount_type: MeteredUnit`.
    pub cap_amount: Option<u32>,
}

#[derive(Default, Deserialize, Serialize)]
pub struct UpdateProductParams {
    /// Key-value object allowing you to store additional information.
    pub metadata: HashMap<String, String>,
    /// The name of the product.
    ///
    /// Minimum length: `3`
    pub name: Option<String>,
    /// The description of the product.
    pub description: Option<String>,
    /// The recurring interval of the product. If `None`, the product is a one-time purchase
    pub recurring_interval: Option<RecurringInterval>,
    /// Whether the product is archived. If `true`, the product won't be available for purchase anymore. Existing customers will still have access to their benefits, and subscriptions will continue normally.
    pub is_archived: Option<bool>,
    /// List of available prices for this product. If you want to keep existing prices, include them in the list with only the `id`.
    pub prices: Vec<UpdatePriceParams>,
    /// List of file IDs. Each one must be on the same organization as the product, of type `product_media` and correctly uploaded.
    pub medias: Option<Vec<Uuid>>,
    /// List of custom fields to attach.
    pub attached_custom_fields: Option<Vec<AttachedCustomFieldParams>>,
}
