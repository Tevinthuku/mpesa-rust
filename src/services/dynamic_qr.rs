#![doc = include_str!("../../docs/client/dynamic_qr.md")]

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::client::Mpesa;
use crate::constants::TransactionType;
use crate::environment::ApiEnvironment;
use crate::errors::{MpesaError, MpesaResult};

const DYNAMIC_QR_URL: &str = "mpesa/qrcode/v1/generate";

#[derive(Debug, Serialize)]
#[serde(rename_all(serialize = "PascalCase"))]
pub struct DynamicQRRequest<'mpesa> {
    /// Name of the Company/M-Pesa Merchant Name
    pub merchant_name: &'mpesa str,
    /// Transaction Reference Number
    pub ref_no: &'mpesa str,
    /// The total amount of the transaction
    pub amount: u32,
    /// Transaction Type
    ///
    /// This can be a `TransactionType` or a `&str`
    /// The `&str` must be one of the following:
    /// - `BG` for Buy Goods
    /// - `PB` for Pay Bill
    /// - `WA` Withdraw Cash
    /// - `SM` Send Money (Mobile Number)
    /// - `SB` Sent to Business. Business number CPI in MSISDN format.
    #[serde(rename = "TrxCode")]
    pub transaction_type: TransactionType,
    ///Credit Party Identifier.
    ///
    /// Can be a Mobile Number, Business Number, Agent
    /// Till, Paybill or Business number, or Merchant Buy Goods.
    #[serde(rename = "CPI")]
    pub credit_party_identifier: &'mpesa str,
    /// Size of the QR code image in pixels.
    ///
    /// QR code image will always be a square image.
    pub size: &'mpesa str,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct DynamicQRResponse {
    #[serde(rename(deserialize = "QRCode"))]
    pub qr_code: String,
    pub response_code: String,
    pub response_description: String,
}

/// Dynamic QR builder struct
#[derive(Builder, Debug, Clone)]
#[builder(build_fn(error = "MpesaError"))]
pub struct DynamicQR<'mpesa, Env: ApiEnvironment> {
    #[builder(pattern = "immutable")]
    client: &'mpesa Mpesa<Env>,
    /// Name of the Company/M-Pesa Merchant Name
    #[builder(setter(into))]
    merchant_name: &'mpesa str,
    /// Transaction Reference Number
    amount: u32,
    /// The total amount of the transaction
    ref_no: &'mpesa str,
    /// Transaction Type
    ///
    /// This can be a `TransactionType` or a `&str`
    /// The `&str` must be one of the following:
    /// - `BG` for Buy Goods
    /// - `PB` for Pay Bill
    /// - `WA` Withdraw Cash
    /// - `SM` Send Money (Mobile Number)
    /// - `SB` Sent to Business. Business number CPI in MSISDN format.
    #[builder(try_setter, setter(into))]
    transaction_type: TransactionType,
    /// Credit Party Identifier.
    /// Can be a Mobile Number, Business Number, Agent
    /// Till, Paybill or Business number, or Merchant Buy Goods.
    #[builder(setter(into))]
    credit_party_identifier: &'mpesa str,
    /// Size of the QR code image in pixels.
    ///
    /// QR code image will always be a square image.
    #[builder(setter(into))]
    size: &'mpesa str,
}

impl<'mpesa, Env: ApiEnvironment> From<DynamicQR<'mpesa, Env>> for DynamicQRRequest<'mpesa> {
    fn from(express: DynamicQR<'mpesa, Env>) -> DynamicQRRequest<'mpesa> {
        DynamicQRRequest {
            merchant_name: express.merchant_name,
            ref_no: express.ref_no,
            amount: express.amount,
            transaction_type: express.transaction_type,
            credit_party_identifier: express.credit_party_identifier,
            size: express.size,
        }
    }
}

impl<'mpesa, Env: ApiEnvironment> DynamicQR<'mpesa, Env> {
    pub(crate) fn builder(client: &'mpesa Mpesa<Env>) -> DynamicQRBuilder<'mpesa, Env> {
        DynamicQRBuilder::default().client(client)
    }

    /// # Build Dynamic QR
    ///
    /// Returns a `DynamicQR` which can be used to send a request
    pub fn from_request(
        client: &'mpesa Mpesa<Env>,
        request: DynamicQRRequest<'mpesa>,
    ) -> DynamicQR<'mpesa, Env> {
        DynamicQR {
            client,
            merchant_name: request.merchant_name,
            ref_no: request.ref_no,
            amount: request.amount,
            transaction_type: request.transaction_type,
            credit_party_identifier: request.credit_party_identifier,
            size: request.size,
        }
    }

    /// # Generate a Dynamic QR
    ///
    /// This enables Safaricom M-PESA customers who
    /// have My Safaricom App or M-PESA app, to scan a QR (Quick Response)
    /// code, to capture till number and amount then authorize to pay for goods
    /// and services at select LIPA NA M-PESA (LNM) merchant outlets.
    ///
    /// # Response
    /// A successful request returns a `DynamicQRResponse` type
    /// which contains the QR code
    ///
    /// # Errors
    /// Returns a `MpesaError` on failure
    pub async fn send(self) -> MpesaResult<DynamicQRResponse> {
        self.client
            .send::<DynamicQRRequest, _>(crate::client::Request {
                method: reqwest::Method::POST,
                path: DYNAMIC_QR_URL,
                body: self.into(),
            })
            .await
    }
}
