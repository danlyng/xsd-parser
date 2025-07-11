use xsd_parser::{
    models::schema::Namespace,
    quick_xml::{Error, WithDeserializer, WithSerializer},
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_RSM: Namespace =
    Namespace::new_const(b"urn:un:unece:uncefact:data:standard:CrossIndustryInvoice:100");
pub const NS_QDT: Namespace =
    Namespace::new_const(b"urn:un:unece:uncefact:data:standard:QualifiedDataType:100");
pub const NS_RAM: Namespace = Namespace::new_const(
    b"urn:un:unece:uncefact:data:standard:ReusableAggregateBusinessInformationEntity:100",
);
pub const NS_UDT: Namespace =
    Namespace::new_const(b"urn:un:unece:uncefact:data:standard:UnqualifiedDataType:100");
pub type CrossIndustryInvoice = CrossIndustryInvoiceType;
#[derive(Debug)]
pub struct CrossIndustryInvoiceType {
    pub exchanged_document_context: ExchangedDocumentContextType,
    pub exchanged_document: ExchangedDocumentType,
    pub supply_chain_trade_transaction: SupplyChainTradeTransactionType,
}
impl WithSerializer for CrossIndustryInvoiceType {
    type Serializer<'x> = quick_xml_serialize::CrossIndustryInvoiceTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::CrossIndustryInvoiceTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::CrossIndustryInvoiceTypeSerializerState::Init__),
            name: name.unwrap_or("rsm:CrossIndustryInvoiceType"),
            is_root,
        })
    }
}
impl WithDeserializer for CrossIndustryInvoiceType {
    type Deserializer = quick_xml_deserialize::CrossIndustryInvoiceTypeDeserializer;
}
#[derive(Debug)]
pub struct ExchangedDocumentContextType {
    pub business_process_specified_document_context_parameter: Option<DocumentContextParameterType>,
    pub guideline_specified_document_context_parameter: DocumentContextParameterType,
}
impl WithSerializer for ExchangedDocumentContextType {
    type Serializer<'x> = quick_xml_serialize::ExchangedDocumentContextTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(
            quick_xml_serialize::ExchangedDocumentContextTypeSerializer {
                value: self,
                state: Box::new(
                    quick_xml_serialize::ExchangedDocumentContextTypeSerializerState::Init__,
                ),
                name: name.unwrap_or("ram:ExchangedDocumentContextType"),
                is_root,
            },
        )
    }
}
impl WithDeserializer for ExchangedDocumentContextType {
    type Deserializer = quick_xml_deserialize::ExchangedDocumentContextTypeDeserializer;
}
#[derive(Debug)]
pub struct ExchangedDocumentType {
    pub id: IdType,
    pub type_code: DocumentCodeType,
    pub issue_date_time: DateTimeType,
    pub included_note: Vec<NoteType>,
}
impl WithSerializer for ExchangedDocumentType {
    type Serializer<'x> = quick_xml_serialize::ExchangedDocumentTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::ExchangedDocumentTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::ExchangedDocumentTypeSerializerState::Init__),
            name: name.unwrap_or("ram:ExchangedDocumentType"),
            is_root,
        })
    }
}
impl WithDeserializer for ExchangedDocumentType {
    type Deserializer = quick_xml_deserialize::ExchangedDocumentTypeDeserializer;
}
#[derive(Debug)]
pub struct SupplyChainTradeTransactionType {
    pub applicable_header_trade_agreement: HeaderTradeAgreementType,
    pub applicable_header_trade_delivery: HeaderTradeDeliveryType,
    pub applicable_header_trade_settlement: HeaderTradeSettlementType,
}
impl WithSerializer for SupplyChainTradeTransactionType {
    type Serializer<'x> = quick_xml_serialize::SupplyChainTradeTransactionTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(
            quick_xml_serialize::SupplyChainTradeTransactionTypeSerializer {
                value: self,
                state: Box::new(
                    quick_xml_serialize::SupplyChainTradeTransactionTypeSerializerState::Init__,
                ),
                name: name.unwrap_or("ram:SupplyChainTradeTransactionType"),
                is_root,
            },
        )
    }
}
impl WithDeserializer for SupplyChainTradeTransactionType {
    type Deserializer = quick_xml_deserialize::SupplyChainTradeTransactionTypeDeserializer;
}
#[derive(Debug)]
pub struct DocumentContextParameterType {
    pub id: IdType,
}
impl WithSerializer for DocumentContextParameterType {
    type Serializer<'x> = quick_xml_serialize::DocumentContextParameterTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(
            quick_xml_serialize::DocumentContextParameterTypeSerializer {
                value: self,
                state: Box::new(
                    quick_xml_serialize::DocumentContextParameterTypeSerializerState::Init__,
                ),
                name: name.unwrap_or("ram:DocumentContextParameterType"),
                is_root,
            },
        )
    }
}
impl WithDeserializer for DocumentContextParameterType {
    type Deserializer = quick_xml_deserialize::DocumentContextParameterTypeDeserializer;
}
#[derive(Debug)]
pub struct IdType {
    pub scheme_id: Option<String>,
    pub content: String,
}
impl WithSerializer for IdType {
    type Serializer<'x> = quick_xml_serialize::IdTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::IdTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::IdTypeSerializerState::Init__),
            name: name.unwrap_or("udt:IDType"),
            is_root,
        })
    }
}
impl WithDeserializer for IdType {
    type Deserializer = quick_xml_deserialize::IdTypeDeserializer;
}
#[derive(Debug)]
pub struct DocumentCodeType {
    pub content: String,
}
impl WithSerializer for DocumentCodeType {
    type Serializer<'x> = quick_xml_serialize::DocumentCodeTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::DocumentCodeTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::DocumentCodeTypeSerializerState::Init__),
            name: name.unwrap_or("qdt:DocumentCodeType"),
            is_root,
        })
    }
}
impl WithDeserializer for DocumentCodeType {
    type Deserializer = quick_xml_deserialize::DocumentCodeTypeDeserializer;
}
#[derive(Debug)]
pub struct DateTimeType {
    pub content: DateTimeTypeContent,
}
#[derive(Debug)]
pub enum DateTimeTypeContent {
    DateTimeString(DateTimeTypeDateTimeStringType),
}
impl WithSerializer for DateTimeType {
    type Serializer<'x> = quick_xml_serialize::DateTimeTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::DateTimeTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::DateTimeTypeSerializerState::Init__),
            name: name.unwrap_or("udt:DateTimeType"),
            is_root,
        })
    }
}
impl WithSerializer for DateTimeTypeContent {
    type Serializer<'x> = quick_xml_serialize::DateTimeTypeContentSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::DateTimeTypeContentSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::DateTimeTypeContentSerializerState::Init__),
        })
    }
}
impl WithDeserializer for DateTimeType {
    type Deserializer = quick_xml_deserialize::DateTimeTypeDeserializer;
}
impl WithDeserializer for DateTimeTypeContent {
    type Deserializer = quick_xml_deserialize::DateTimeTypeContentDeserializer;
}
#[derive(Debug)]
pub struct NoteType {
    pub content: TextType,
    pub subject_code: Option<CodeType>,
}
impl WithSerializer for NoteType {
    type Serializer<'x> = quick_xml_serialize::NoteTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::NoteTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::NoteTypeSerializerState::Init__),
            name: name.unwrap_or("ram:NoteType"),
            is_root,
        })
    }
}
impl WithDeserializer for NoteType {
    type Deserializer = quick_xml_deserialize::NoteTypeDeserializer;
}
#[derive(Debug)]
pub struct HeaderTradeAgreementType {
    pub buyer_reference: Option<TextType>,
    pub seller_trade_party: TradePartyType,
    pub buyer_trade_party: TradePartyType,
    pub seller_tax_representative_trade_party: Option<TradePartyType>,
    pub buyer_order_referenced_document: Option<ReferencedDocumentType>,
    pub contract_referenced_document: Option<ReferencedDocumentType>,
}
impl WithSerializer for HeaderTradeAgreementType {
    type Serializer<'x> = quick_xml_serialize::HeaderTradeAgreementTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::HeaderTradeAgreementTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::HeaderTradeAgreementTypeSerializerState::Init__),
            name: name.unwrap_or("ram:HeaderTradeAgreementType"),
            is_root,
        })
    }
}
impl WithDeserializer for HeaderTradeAgreementType {
    type Deserializer = quick_xml_deserialize::HeaderTradeAgreementTypeDeserializer;
}
#[derive(Debug)]
pub struct HeaderTradeDeliveryType {
    pub ship_to_trade_party: Option<TradePartyType>,
    pub actual_delivery_supply_chain_event: Option<SupplyChainEventType>,
    pub despatch_advice_referenced_document: Option<ReferencedDocumentType>,
}
impl WithSerializer for HeaderTradeDeliveryType {
    type Serializer<'x> = quick_xml_serialize::HeaderTradeDeliveryTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::HeaderTradeDeliveryTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::HeaderTradeDeliveryTypeSerializerState::Init__),
            name: name.unwrap_or("ram:HeaderTradeDeliveryType"),
            is_root,
        })
    }
}
impl WithDeserializer for HeaderTradeDeliveryType {
    type Deserializer = quick_xml_deserialize::HeaderTradeDeliveryTypeDeserializer;
}
#[derive(Debug)]
pub struct HeaderTradeSettlementType {
    pub creditor_reference_id: Option<IdType>,
    pub payment_reference: Option<TextType>,
    pub tax_currency_code: Option<CurrencyCodeType>,
    pub invoice_currency_code: CurrencyCodeType,
    pub payee_trade_party: Option<TradePartyType>,
    pub specified_trade_settlement_payment_means: Vec<TradeSettlementPaymentMeansType>,
    pub applicable_trade_tax: Vec<TradeTaxType>,
    pub billing_specified_period: Option<SpecifiedPeriodType>,
    pub specified_trade_allowance_charge: Vec<TradeAllowanceChargeType>,
    pub specified_trade_payment_terms: Option<TradePaymentTermsType>,
    pub specified_trade_settlement_header_monetary_summation:
        TradeSettlementHeaderMonetarySummationType,
    pub invoice_referenced_document: Vec<ReferencedDocumentType>,
    pub receivable_specified_trade_accounting_account: Option<TradeAccountingAccountType>,
}
impl WithSerializer for HeaderTradeSettlementType {
    type Serializer<'x> = quick_xml_serialize::HeaderTradeSettlementTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::HeaderTradeSettlementTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::HeaderTradeSettlementTypeSerializerState::Init__),
            name: name.unwrap_or("ram:HeaderTradeSettlementType"),
            is_root,
        })
    }
}
impl WithDeserializer for HeaderTradeSettlementType {
    type Deserializer = quick_xml_deserialize::HeaderTradeSettlementTypeDeserializer;
}
#[derive(Debug)]
pub struct DateTimeTypeDateTimeStringType {
    pub format: String,
    pub content: String,
}
impl WithSerializer for DateTimeTypeDateTimeStringType {
    type Serializer<'x> = quick_xml_serialize::DateTimeTypeDateTimeStringTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(
            quick_xml_serialize::DateTimeTypeDateTimeStringTypeSerializer {
                value: self,
                state: Box::new(
                    quick_xml_serialize::DateTimeTypeDateTimeStringTypeSerializerState::Init__,
                ),
                name: name.unwrap_or("udt:DateTimeTypeDateTimeString"),
                is_root,
            },
        )
    }
}
impl WithDeserializer for DateTimeTypeDateTimeStringType {
    type Deserializer = quick_xml_deserialize::DateTimeTypeDateTimeStringTypeDeserializer;
}
#[derive(Debug)]
pub struct TextType {
    pub content: String,
}
impl WithSerializer for TextType {
    type Serializer<'x> = quick_xml_serialize::TextTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::TextTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::TextTypeSerializerState::Init__),
            name: name.unwrap_or("udt:TextType"),
            is_root,
        })
    }
}
impl WithDeserializer for TextType {
    type Deserializer = quick_xml_deserialize::TextTypeDeserializer;
}
#[derive(Debug)]
pub struct CodeType {
    pub content: String,
}
impl WithSerializer for CodeType {
    type Serializer<'x> = quick_xml_serialize::CodeTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::CodeTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::CodeTypeSerializerState::Init__),
            name: name.unwrap_or("udt:CodeType"),
            is_root,
        })
    }
}
impl WithDeserializer for CodeType {
    type Deserializer = quick_xml_deserialize::CodeTypeDeserializer;
}
#[derive(Debug)]
pub struct TradePartyType {
    pub id: Vec<IdType>,
    pub global_id: Vec<IdType>,
    pub name: Option<TextType>,
    pub specified_legal_organization: Option<LegalOrganizationType>,
    pub postal_trade_address: Option<TradeAddressType>,
    pub uri_universal_communication: Option<UniversalCommunicationType>,
    pub specified_tax_registration: Vec<TaxRegistrationType>,
}
impl WithSerializer for TradePartyType {
    type Serializer<'x> = quick_xml_serialize::TradePartyTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::TradePartyTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::TradePartyTypeSerializerState::Init__),
            name: name.unwrap_or("ram:TradePartyType"),
            is_root,
        })
    }
}
impl WithDeserializer for TradePartyType {
    type Deserializer = quick_xml_deserialize::TradePartyTypeDeserializer;
}
#[derive(Debug)]
pub struct ReferencedDocumentType {
    pub issuer_assigned_id: IdType,
    pub formatted_issue_date_time: Option<FormattedDateTimeType>,
}
impl WithSerializer for ReferencedDocumentType {
    type Serializer<'x> = quick_xml_serialize::ReferencedDocumentTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::ReferencedDocumentTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::ReferencedDocumentTypeSerializerState::Init__),
            name: name.unwrap_or("ram:ReferencedDocumentType"),
            is_root,
        })
    }
}
impl WithDeserializer for ReferencedDocumentType {
    type Deserializer = quick_xml_deserialize::ReferencedDocumentTypeDeserializer;
}
#[derive(Debug)]
pub struct SupplyChainEventType {
    pub occurrence_date_time: DateTimeType,
}
impl WithSerializer for SupplyChainEventType {
    type Serializer<'x> = quick_xml_serialize::SupplyChainEventTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::SupplyChainEventTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::SupplyChainEventTypeSerializerState::Init__),
            name: name.unwrap_or("ram:SupplyChainEventType"),
            is_root,
        })
    }
}
impl WithDeserializer for SupplyChainEventType {
    type Deserializer = quick_xml_deserialize::SupplyChainEventTypeDeserializer;
}
#[derive(Debug)]
pub struct CurrencyCodeType {
    pub content: String,
}
impl WithSerializer for CurrencyCodeType {
    type Serializer<'x> = quick_xml_serialize::CurrencyCodeTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::CurrencyCodeTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::CurrencyCodeTypeSerializerState::Init__),
            name: name.unwrap_or("qdt:CurrencyCodeType"),
            is_root,
        })
    }
}
impl WithDeserializer for CurrencyCodeType {
    type Deserializer = quick_xml_deserialize::CurrencyCodeTypeDeserializer;
}
#[derive(Debug)]
pub struct TradeSettlementPaymentMeansType {
    pub type_code: PaymentMeansCodeType,
    pub payer_party_debtor_financial_account: Option<DebtorFinancialAccountType>,
    pub payee_party_creditor_financial_account: Option<CreditorFinancialAccountType>,
}
impl WithSerializer for TradeSettlementPaymentMeansType {
    type Serializer<'x> = quick_xml_serialize::TradeSettlementPaymentMeansTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(
            quick_xml_serialize::TradeSettlementPaymentMeansTypeSerializer {
                value: self,
                state: Box::new(
                    quick_xml_serialize::TradeSettlementPaymentMeansTypeSerializerState::Init__,
                ),
                name: name.unwrap_or("ram:TradeSettlementPaymentMeansType"),
                is_root,
            },
        )
    }
}
impl WithDeserializer for TradeSettlementPaymentMeansType {
    type Deserializer = quick_xml_deserialize::TradeSettlementPaymentMeansTypeDeserializer;
}
#[derive(Debug)]
pub struct TradeTaxType {
    pub calculated_amount: Option<AmountType>,
    pub type_code: TaxTypeCodeType,
    pub exemption_reason: Option<TextType>,
    pub basis_amount: Option<AmountType>,
    pub category_code: TaxCategoryCodeType,
    pub exemption_reason_code: Option<CodeType>,
    pub due_date_type_code: Option<TimeReferenceCodeType>,
    pub rate_applicable_percent: Option<PercentType>,
}
impl WithSerializer for TradeTaxType {
    type Serializer<'x> = quick_xml_serialize::TradeTaxTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::TradeTaxTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::TradeTaxTypeSerializerState::Init__),
            name: name.unwrap_or("ram:TradeTaxType"),
            is_root,
        })
    }
}
impl WithDeserializer for TradeTaxType {
    type Deserializer = quick_xml_deserialize::TradeTaxTypeDeserializer;
}
#[derive(Debug)]
pub struct SpecifiedPeriodType {
    pub start_date_time: Option<DateTimeType>,
    pub end_date_time: Option<DateTimeType>,
}
impl WithSerializer for SpecifiedPeriodType {
    type Serializer<'x> = quick_xml_serialize::SpecifiedPeriodTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::SpecifiedPeriodTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::SpecifiedPeriodTypeSerializerState::Init__),
            name: name.unwrap_or("ram:SpecifiedPeriodType"),
            is_root,
        })
    }
}
impl WithDeserializer for SpecifiedPeriodType {
    type Deserializer = quick_xml_deserialize::SpecifiedPeriodTypeDeserializer;
}
#[derive(Debug)]
pub struct TradeAllowanceChargeType {
    pub charge_indicator: IndicatorType,
    pub calculation_percent: Option<PercentType>,
    pub basis_amount: Option<AmountType>,
    pub actual_amount: AmountType,
    pub reason_code: Option<AllowanceChargeReasonCodeType>,
    pub reason: Option<TextType>,
    pub category_trade_tax: TradeTaxType,
}
impl WithSerializer for TradeAllowanceChargeType {
    type Serializer<'x> = quick_xml_serialize::TradeAllowanceChargeTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::TradeAllowanceChargeTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::TradeAllowanceChargeTypeSerializerState::Init__),
            name: name.unwrap_or("ram:TradeAllowanceChargeType"),
            is_root,
        })
    }
}
impl WithDeserializer for TradeAllowanceChargeType {
    type Deserializer = quick_xml_deserialize::TradeAllowanceChargeTypeDeserializer;
}
#[derive(Debug)]
pub struct TradePaymentTermsType {
    pub description: Option<TextType>,
    pub due_date_date_time: Option<DateTimeType>,
    pub direct_debit_mandate_id: Option<IdType>,
}
impl WithSerializer for TradePaymentTermsType {
    type Serializer<'x> = quick_xml_serialize::TradePaymentTermsTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::TradePaymentTermsTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::TradePaymentTermsTypeSerializerState::Init__),
            name: name.unwrap_or("ram:TradePaymentTermsType"),
            is_root,
        })
    }
}
impl WithDeserializer for TradePaymentTermsType {
    type Deserializer = quick_xml_deserialize::TradePaymentTermsTypeDeserializer;
}
#[derive(Debug)]
pub struct TradeSettlementHeaderMonetarySummationType {
    pub line_total_amount: AmountType,
    pub charge_total_amount: Option<AmountType>,
    pub allowance_total_amount: Option<AmountType>,
    pub tax_basis_total_amount: AmountType,
    pub tax_total_amount: Vec<AmountType>,
    pub grand_total_amount: AmountType,
    pub total_prepaid_amount: Option<AmountType>,
    pub due_payable_amount: AmountType,
}
impl WithSerializer for TradeSettlementHeaderMonetarySummationType {
    type Serializer<'x> =
        quick_xml_serialize::TradeSettlementHeaderMonetarySummationTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok (quick_xml_serialize :: TradeSettlementHeaderMonetarySummationTypeSerializer { value : self , state : Box :: new (quick_xml_serialize :: TradeSettlementHeaderMonetarySummationTypeSerializerState :: Init__) , name : name . unwrap_or ("ram:TradeSettlementHeaderMonetarySummationType") , is_root , })
    }
}
impl WithDeserializer for TradeSettlementHeaderMonetarySummationType {
    type Deserializer =
        quick_xml_deserialize::TradeSettlementHeaderMonetarySummationTypeDeserializer;
}
#[derive(Debug)]
pub struct TradeAccountingAccountType {
    pub id: IdType,
}
impl WithSerializer for TradeAccountingAccountType {
    type Serializer<'x> = quick_xml_serialize::TradeAccountingAccountTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::TradeAccountingAccountTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::TradeAccountingAccountTypeSerializerState::Init__),
            name: name.unwrap_or("ram:TradeAccountingAccountType"),
            is_root,
        })
    }
}
impl WithDeserializer for TradeAccountingAccountType {
    type Deserializer = quick_xml_deserialize::TradeAccountingAccountTypeDeserializer;
}
#[derive(Debug)]
pub struct LegalOrganizationType {
    pub id: Option<IdType>,
    pub trading_business_name: Option<TextType>,
}
impl WithSerializer for LegalOrganizationType {
    type Serializer<'x> = quick_xml_serialize::LegalOrganizationTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::LegalOrganizationTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::LegalOrganizationTypeSerializerState::Init__),
            name: name.unwrap_or("ram:LegalOrganizationType"),
            is_root,
        })
    }
}
impl WithDeserializer for LegalOrganizationType {
    type Deserializer = quick_xml_deserialize::LegalOrganizationTypeDeserializer;
}
#[derive(Debug)]
pub struct TradeAddressType {
    pub postcode_code: Option<CodeType>,
    pub line_one: Option<TextType>,
    pub line_two: Option<TextType>,
    pub line_three: Option<TextType>,
    pub city_name: Option<TextType>,
    pub country_id: CountryIdType,
    pub country_sub_division_name: Option<TextType>,
}
impl WithSerializer for TradeAddressType {
    type Serializer<'x> = quick_xml_serialize::TradeAddressTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::TradeAddressTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::TradeAddressTypeSerializerState::Init__),
            name: name.unwrap_or("ram:TradeAddressType"),
            is_root,
        })
    }
}
impl WithDeserializer for TradeAddressType {
    type Deserializer = quick_xml_deserialize::TradeAddressTypeDeserializer;
}
#[derive(Debug)]
pub struct UniversalCommunicationType {
    pub uriid: IdType,
}
impl WithSerializer for UniversalCommunicationType {
    type Serializer<'x> = quick_xml_serialize::UniversalCommunicationTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::UniversalCommunicationTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::UniversalCommunicationTypeSerializerState::Init__),
            name: name.unwrap_or("ram:UniversalCommunicationType"),
            is_root,
        })
    }
}
impl WithDeserializer for UniversalCommunicationType {
    type Deserializer = quick_xml_deserialize::UniversalCommunicationTypeDeserializer;
}
#[derive(Debug)]
pub struct TaxRegistrationType {
    pub id: IdType,
}
impl WithSerializer for TaxRegistrationType {
    type Serializer<'x> = quick_xml_serialize::TaxRegistrationTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::TaxRegistrationTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::TaxRegistrationTypeSerializerState::Init__),
            name: name.unwrap_or("ram:TaxRegistrationType"),
            is_root,
        })
    }
}
impl WithDeserializer for TaxRegistrationType {
    type Deserializer = quick_xml_deserialize::TaxRegistrationTypeDeserializer;
}
#[derive(Debug)]
pub struct FormattedDateTimeType {
    pub date_time_string: FormattedDateTimeTypeDateTimeStringType,
}
impl WithSerializer for FormattedDateTimeType {
    type Serializer<'x> = quick_xml_serialize::FormattedDateTimeTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::FormattedDateTimeTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::FormattedDateTimeTypeSerializerState::Init__),
            name: name.unwrap_or("qdt:FormattedDateTimeType"),
            is_root,
        })
    }
}
impl WithDeserializer for FormattedDateTimeType {
    type Deserializer = quick_xml_deserialize::FormattedDateTimeTypeDeserializer;
}
#[derive(Debug)]
pub struct PaymentMeansCodeType {
    pub content: String,
}
impl WithSerializer for PaymentMeansCodeType {
    type Serializer<'x> = quick_xml_serialize::PaymentMeansCodeTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::PaymentMeansCodeTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::PaymentMeansCodeTypeSerializerState::Init__),
            name: name.unwrap_or("qdt:PaymentMeansCodeType"),
            is_root,
        })
    }
}
impl WithDeserializer for PaymentMeansCodeType {
    type Deserializer = quick_xml_deserialize::PaymentMeansCodeTypeDeserializer;
}
#[derive(Debug)]
pub struct DebtorFinancialAccountType {
    pub ibanid: IdType,
}
impl WithSerializer for DebtorFinancialAccountType {
    type Serializer<'x> = quick_xml_serialize::DebtorFinancialAccountTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::DebtorFinancialAccountTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::DebtorFinancialAccountTypeSerializerState::Init__),
            name: name.unwrap_or("ram:DebtorFinancialAccountType"),
            is_root,
        })
    }
}
impl WithDeserializer for DebtorFinancialAccountType {
    type Deserializer = quick_xml_deserialize::DebtorFinancialAccountTypeDeserializer;
}
#[derive(Debug)]
pub struct CreditorFinancialAccountType {
    pub ibanid: Option<IdType>,
    pub proprietary_id: Option<IdType>,
}
impl WithSerializer for CreditorFinancialAccountType {
    type Serializer<'x> = quick_xml_serialize::CreditorFinancialAccountTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(
            quick_xml_serialize::CreditorFinancialAccountTypeSerializer {
                value: self,
                state: Box::new(
                    quick_xml_serialize::CreditorFinancialAccountTypeSerializerState::Init__,
                ),
                name: name.unwrap_or("ram:CreditorFinancialAccountType"),
                is_root,
            },
        )
    }
}
impl WithDeserializer for CreditorFinancialAccountType {
    type Deserializer = quick_xml_deserialize::CreditorFinancialAccountTypeDeserializer;
}
#[derive(Debug)]
pub struct AmountType {
    pub currency_id: Option<String>,
    pub content: f64,
}
impl WithSerializer for AmountType {
    type Serializer<'x> = quick_xml_serialize::AmountTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::AmountTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::AmountTypeSerializerState::Init__),
            name: name.unwrap_or("udt:AmountType"),
            is_root,
        })
    }
}
impl WithDeserializer for AmountType {
    type Deserializer = quick_xml_deserialize::AmountTypeDeserializer;
}
#[derive(Debug)]
pub struct TaxTypeCodeType {
    pub content: String,
}
impl WithSerializer for TaxTypeCodeType {
    type Serializer<'x> = quick_xml_serialize::TaxTypeCodeTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::TaxTypeCodeTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::TaxTypeCodeTypeSerializerState::Init__),
            name: name.unwrap_or("qdt:TaxTypeCodeType"),
            is_root,
        })
    }
}
impl WithDeserializer for TaxTypeCodeType {
    type Deserializer = quick_xml_deserialize::TaxTypeCodeTypeDeserializer;
}
#[derive(Debug)]
pub struct TaxCategoryCodeType {
    pub content: String,
}
impl WithSerializer for TaxCategoryCodeType {
    type Serializer<'x> = quick_xml_serialize::TaxCategoryCodeTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::TaxCategoryCodeTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::TaxCategoryCodeTypeSerializerState::Init__),
            name: name.unwrap_or("qdt:TaxCategoryCodeType"),
            is_root,
        })
    }
}
impl WithDeserializer for TaxCategoryCodeType {
    type Deserializer = quick_xml_deserialize::TaxCategoryCodeTypeDeserializer;
}
#[derive(Debug)]
pub struct TimeReferenceCodeType {
    pub content: String,
}
impl WithSerializer for TimeReferenceCodeType {
    type Serializer<'x> = quick_xml_serialize::TimeReferenceCodeTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::TimeReferenceCodeTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::TimeReferenceCodeTypeSerializerState::Init__),
            name: name.unwrap_or("qdt:TimeReferenceCodeType"),
            is_root,
        })
    }
}
impl WithDeserializer for TimeReferenceCodeType {
    type Deserializer = quick_xml_deserialize::TimeReferenceCodeTypeDeserializer;
}
#[derive(Debug)]
pub struct PercentType {
    pub content: f64,
}
impl WithSerializer for PercentType {
    type Serializer<'x> = quick_xml_serialize::PercentTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::PercentTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::PercentTypeSerializerState::Init__),
            name: name.unwrap_or("udt:PercentType"),
            is_root,
        })
    }
}
impl WithDeserializer for PercentType {
    type Deserializer = quick_xml_deserialize::PercentTypeDeserializer;
}
#[derive(Debug)]
pub struct IndicatorType {
    pub content: IndicatorTypeContent,
}
#[derive(Debug)]
pub enum IndicatorTypeContent {
    Indicator(bool),
}
impl WithSerializer for IndicatorType {
    type Serializer<'x> = quick_xml_serialize::IndicatorTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::IndicatorTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::IndicatorTypeSerializerState::Init__),
            name: name.unwrap_or("udt:IndicatorType"),
            is_root,
        })
    }
}
impl WithSerializer for IndicatorTypeContent {
    type Serializer<'x> = quick_xml_serialize::IndicatorTypeContentSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        let _name = name;
        let _is_root = is_root;
        Ok(quick_xml_serialize::IndicatorTypeContentSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::IndicatorTypeContentSerializerState::Init__),
        })
    }
}
impl WithDeserializer for IndicatorType {
    type Deserializer = quick_xml_deserialize::IndicatorTypeDeserializer;
}
impl WithDeserializer for IndicatorTypeContent {
    type Deserializer = quick_xml_deserialize::IndicatorTypeContentDeserializer;
}
#[derive(Debug)]
pub struct AllowanceChargeReasonCodeType {
    pub content: String,
}
impl WithSerializer for AllowanceChargeReasonCodeType {
    type Serializer<'x> = quick_xml_serialize::AllowanceChargeReasonCodeTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(
            quick_xml_serialize::AllowanceChargeReasonCodeTypeSerializer {
                value: self,
                state: Box::new(
                    quick_xml_serialize::AllowanceChargeReasonCodeTypeSerializerState::Init__,
                ),
                name: name.unwrap_or("qdt:AllowanceChargeReasonCodeType"),
                is_root,
            },
        )
    }
}
impl WithDeserializer for AllowanceChargeReasonCodeType {
    type Deserializer = quick_xml_deserialize::AllowanceChargeReasonCodeTypeDeserializer;
}
#[derive(Debug)]
pub struct CountryIdType {
    pub content: String,
}
impl WithSerializer for CountryIdType {
    type Serializer<'x> = quick_xml_serialize::CountryIdTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::CountryIdTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::CountryIdTypeSerializerState::Init__),
            name: name.unwrap_or("qdt:CountryIDType"),
            is_root,
        })
    }
}
impl WithDeserializer for CountryIdType {
    type Deserializer = quick_xml_deserialize::CountryIdTypeDeserializer;
}
#[derive(Debug)]
pub struct FormattedDateTimeTypeDateTimeStringType {
    pub format: String,
    pub content: String,
}
impl WithSerializer for FormattedDateTimeTypeDateTimeStringType {
    type Serializer<'x> =
        quick_xml_serialize::FormattedDateTimeTypeDateTimeStringTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok (quick_xml_serialize :: FormattedDateTimeTypeDateTimeStringTypeSerializer { value : self , state : Box :: new (quick_xml_serialize :: FormattedDateTimeTypeDateTimeStringTypeSerializerState :: Init__) , name : name . unwrap_or ("qdt:FormattedDateTimeTypeDateTimeString") , is_root , })
    }
}
impl WithDeserializer for FormattedDateTimeTypeDateTimeStringType {
    type Deserializer = quick_xml_deserialize::FormattedDateTimeTypeDateTimeStringTypeDeserializer;
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser::quick_xml::{
        filter_xmlns_attributes, BytesStart, ContentDeserializer, DeserializeReader, Deserializer,
        DeserializerArtifact, DeserializerEvent, DeserializerOutput, DeserializerResult,
        ElementHandlerOutput, Error, ErrorKind, Event, RawByteStr, WithDeserializer,
    };
    #[derive(Debug)]
    pub struct CrossIndustryInvoiceTypeDeserializer {
        exchanged_document_context: Option<super::ExchangedDocumentContextType>,
        exchanged_document: Option<super::ExchangedDocumentType>,
        supply_chain_trade_transaction: Option<super::SupplyChainTradeTransactionType>,
        state: Box<CrossIndustryInvoiceTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum CrossIndustryInvoiceTypeDeserializerState {
        Init__,
        ExchangedDocumentContext(
            Option<<super::ExchangedDocumentContextType as WithDeserializer>::Deserializer>,
        ),
        ExchangedDocument(Option<<super::ExchangedDocumentType as WithDeserializer>::Deserializer>),
        SupplyChainTradeTransaction(
            Option<<super::SupplyChainTradeTransactionType as WithDeserializer>::Deserializer>,
        ),
        Done__,
        Unknown__,
    }
    impl CrossIndustryInvoiceTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
            }
            Ok(Self {
                exchanged_document_context: None,
                exchanged_document: None,
                supply_chain_trade_transaction: None,
                state: Box::new(CrossIndustryInvoiceTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: CrossIndustryInvoiceTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use CrossIndustryInvoiceTypeDeserializerState as S;
            match state {
                S::ExchangedDocumentContext(Some(deserializer)) => {
                    self.store_exchanged_document_context(deserializer.finish(reader)?)?
                }
                S::ExchangedDocument(Some(deserializer)) => {
                    self.store_exchanged_document(deserializer.finish(reader)?)?
                }
                S::SupplyChainTradeTransaction(Some(deserializer)) => {
                    self.store_supply_chain_trade_transaction(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_exchanged_document_context(
            &mut self,
            value: super::ExchangedDocumentContextType,
        ) -> Result<(), Error> {
            if self.exchanged_document_context.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"ExchangedDocumentContext",
                )))?;
            }
            self.exchanged_document_context = Some(value);
            Ok(())
        }
        fn store_exchanged_document(
            &mut self,
            value: super::ExchangedDocumentType,
        ) -> Result<(), Error> {
            if self.exchanged_document.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"ExchangedDocument",
                )))?;
            }
            self.exchanged_document = Some(value);
            Ok(())
        }
        fn store_supply_chain_trade_transaction(
            &mut self,
            value: super::SupplyChainTradeTransactionType,
        ) -> Result<(), Error> {
            if self.supply_chain_trade_transaction.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"SupplyChainTradeTransaction",
                )))?;
            }
            self.supply_chain_trade_transaction = Some(value);
            Ok(())
        }
        fn handle_exchanged_document_context<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::ExchangedDocumentContextType>,
            fallback: &mut Option<CrossIndustryInvoiceTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.exchanged_document_context.is_some() {
                    fallback.get_or_insert(
                        CrossIndustryInvoiceTypeDeserializerState::ExchangedDocumentContext(None),
                    );
                    *self.state =
                        CrossIndustryInvoiceTypeDeserializerState::ExchangedDocument(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state =
                        CrossIndustryInvoiceTypeDeserializerState::ExchangedDocumentContext(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_exchanged_document_context(data)?;
                    *self.state =
                        CrossIndustryInvoiceTypeDeserializerState::ExchangedDocument(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                CrossIndustryInvoiceTypeDeserializerState::ExchangedDocumentContext(
                                    Some(deserializer),
                                ),
                            );
                            *self.state =
                                CrossIndustryInvoiceTypeDeserializerState::ExchangedDocument(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                CrossIndustryInvoiceTypeDeserializerState::ExchangedDocumentContext(
                                    Some(deserializer),
                                );
                        }
                    }
                    ret
                }
            })
        }
        fn handle_exchanged_document<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::ExchangedDocumentType>,
            fallback: &mut Option<CrossIndustryInvoiceTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.exchanged_document.is_some() {
                    fallback.get_or_insert(
                        CrossIndustryInvoiceTypeDeserializerState::ExchangedDocument(None),
                    );
                    *self.state =
                        CrossIndustryInvoiceTypeDeserializerState::SupplyChainTradeTransaction(
                            None,
                        );
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state =
                        CrossIndustryInvoiceTypeDeserializerState::ExchangedDocument(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_exchanged_document(data)?;
                    *self.state =
                        CrossIndustryInvoiceTypeDeserializerState::SupplyChainTradeTransaction(
                            None,
                        );
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                CrossIndustryInvoiceTypeDeserializerState::ExchangedDocument(Some(
                                    deserializer,
                                )),
                            );
                            * self . state = CrossIndustryInvoiceTypeDeserializerState :: SupplyChainTradeTransaction (None) ;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                CrossIndustryInvoiceTypeDeserializerState::ExchangedDocument(Some(
                                    deserializer,
                                ));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_supply_chain_trade_transaction<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::SupplyChainTradeTransactionType>,
            fallback: &mut Option<CrossIndustryInvoiceTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.supply_chain_trade_transaction.is_some() {
                    fallback.get_or_insert(
                        CrossIndustryInvoiceTypeDeserializerState::SupplyChainTradeTransaction(
                            None,
                        ),
                    );
                    *self.state = CrossIndustryInvoiceTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state =
                        CrossIndustryInvoiceTypeDeserializerState::SupplyChainTradeTransaction(
                            None,
                        );
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_supply_chain_trade_transaction(data)?;
                    *self.state = CrossIndustryInvoiceTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback . get_or_insert (CrossIndustryInvoiceTypeDeserializerState :: SupplyChainTradeTransaction (Some (deserializer))) ;
                            *self.state = CrossIndustryInvoiceTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            * self . state = CrossIndustryInvoiceTypeDeserializerState :: SupplyChainTradeTransaction (Some (deserializer)) ;
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::CrossIndustryInvoiceType>
        for CrossIndustryInvoiceTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::CrossIndustryInvoiceType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::CrossIndustryInvoiceType>
        where
            R: DeserializeReader,
        {
            use CrossIndustryInvoiceTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::ExchangedDocumentContext(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_exchanged_document_context(
                            reader,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::ExchangedDocument(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_exchanged_document(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::SupplyChainTradeTransaction(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_supply_chain_trade_transaction(
                            reader,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state =
                            CrossIndustryInvoiceTypeDeserializerState::ExchangedDocumentContext(
                                None,
                            );
                        event
                    }
                    (
                        S::ExchangedDocumentContext(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RSM),
                            b"ExchangedDocumentContext",
                        ) {
                            let output = < super :: ExchangedDocumentContextType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_exchanged_document_context(
                                reader,
                                output,
                                &mut fallback,
                            )? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::ExchangedDocument(None);
                            event
                        }
                    }
                    (S::ExchangedDocument(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RSM),
                            b"ExchangedDocument",
                        ) {
                            let output = < super :: ExchangedDocumentType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_exchanged_document(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::SupplyChainTradeTransaction(None);
                            event
                        }
                    }
                    (
                        S::SupplyChainTradeTransaction(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RSM),
                            b"SupplyChainTradeTransaction",
                        ) {
                            let output = < super :: SupplyChainTradeTransactionType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_supply_chain_trade_transaction(
                                reader,
                                output,
                                &mut fallback,
                            )? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::CrossIndustryInvoiceType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                CrossIndustryInvoiceTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::CrossIndustryInvoiceType {
                exchanged_document_context: self
                    .exchanged_document_context
                    .ok_or_else(|| ErrorKind::MissingElement("ExchangedDocumentContext".into()))?,
                exchanged_document: self
                    .exchanged_document
                    .ok_or_else(|| ErrorKind::MissingElement("ExchangedDocument".into()))?,
                supply_chain_trade_transaction: self.supply_chain_trade_transaction.ok_or_else(
                    || ErrorKind::MissingElement("SupplyChainTradeTransaction".into()),
                )?,
            })
        }
    }
    #[derive(Debug)]
    pub struct ExchangedDocumentContextTypeDeserializer {
        business_process_specified_document_context_parameter:
            Option<super::DocumentContextParameterType>,
        guideline_specified_document_context_parameter: Option<super::DocumentContextParameterType>,
        state: Box<ExchangedDocumentContextTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ExchangedDocumentContextTypeDeserializerState {
        Init__,
        BusinessProcessSpecifiedDocumentContextParameter(
            Option<<super::DocumentContextParameterType as WithDeserializer>::Deserializer>,
        ),
        GuidelineSpecifiedDocumentContextParameter(
            Option<<super::DocumentContextParameterType as WithDeserializer>::Deserializer>,
        ),
        Done__,
        Unknown__,
    }
    impl ExchangedDocumentContextTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
            }
            Ok(Self {
                business_process_specified_document_context_parameter: None,
                guideline_specified_document_context_parameter: None,
                state: Box::new(ExchangedDocumentContextTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: ExchangedDocumentContextTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use ExchangedDocumentContextTypeDeserializerState as S;
            match state {
                S::BusinessProcessSpecifiedDocumentContextParameter(Some(deserializer)) => self
                    .store_business_process_specified_document_context_parameter(
                        deserializer.finish(reader)?,
                    )?,
                S::GuidelineSpecifiedDocumentContextParameter(Some(deserializer)) => self
                    .store_guideline_specified_document_context_parameter(
                        deserializer.finish(reader)?,
                    )?,
                _ => (),
            }
            Ok(())
        }
        fn store_business_process_specified_document_context_parameter(
            &mut self,
            value: super::DocumentContextParameterType,
        ) -> Result<(), Error> {
            if self
                .business_process_specified_document_context_parameter
                .is_some()
            {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"BusinessProcessSpecifiedDocumentContextParameter",
                )))?;
            }
            self.business_process_specified_document_context_parameter = Some(value);
            Ok(())
        }
        fn store_guideline_specified_document_context_parameter(
            &mut self,
            value: super::DocumentContextParameterType,
        ) -> Result<(), Error> {
            if self
                .guideline_specified_document_context_parameter
                .is_some()
            {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"GuidelineSpecifiedDocumentContextParameter",
                )))?;
            }
            self.guideline_specified_document_context_parameter = Some(value);
            Ok(())
        }
        fn handle_business_process_specified_document_context_parameter<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::DocumentContextParameterType>,
            fallback: &mut Option<ExchangedDocumentContextTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback . get_or_insert (ExchangedDocumentContextTypeDeserializerState :: BusinessProcessSpecifiedDocumentContextParameter (None)) ;
                * self . state = ExchangedDocumentContextTypeDeserializerState :: GuidelineSpecifiedDocumentContextParameter (None) ;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_business_process_specified_document_context_parameter(data)?;
                    * self . state = ExchangedDocumentContextTypeDeserializerState :: GuidelineSpecifiedDocumentContextParameter (None) ;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback . get_or_insert (ExchangedDocumentContextTypeDeserializerState :: BusinessProcessSpecifiedDocumentContextParameter (Some (deserializer))) ;
                            * self . state = ExchangedDocumentContextTypeDeserializerState :: GuidelineSpecifiedDocumentContextParameter (None) ;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            * self . state = ExchangedDocumentContextTypeDeserializerState :: BusinessProcessSpecifiedDocumentContextParameter (Some (deserializer)) ;
                        }
                    }
                    ret
                }
            })
        }
        fn handle_guideline_specified_document_context_parameter<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::DocumentContextParameterType>,
            fallback: &mut Option<ExchangedDocumentContextTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self
                    .guideline_specified_document_context_parameter
                    .is_some()
                {
                    fallback . get_or_insert (ExchangedDocumentContextTypeDeserializerState :: GuidelineSpecifiedDocumentContextParameter (None)) ;
                    *self.state = ExchangedDocumentContextTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    * self . state = ExchangedDocumentContextTypeDeserializerState :: GuidelineSpecifiedDocumentContextParameter (None) ;
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_guideline_specified_document_context_parameter(data)?;
                    *self.state = ExchangedDocumentContextTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback . get_or_insert (ExchangedDocumentContextTypeDeserializerState :: GuidelineSpecifiedDocumentContextParameter (Some (deserializer))) ;
                            *self.state = ExchangedDocumentContextTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            * self . state = ExchangedDocumentContextTypeDeserializerState :: GuidelineSpecifiedDocumentContextParameter (Some (deserializer)) ;
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::ExchangedDocumentContextType>
        for ExchangedDocumentContextTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ExchangedDocumentContextType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ExchangedDocumentContextType>
        where
            R: DeserializeReader,
        {
            use ExchangedDocumentContextTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (
                        S::BusinessProcessSpecifiedDocumentContextParameter(Some(deserializer)),
                        event,
                    ) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_business_process_specified_document_context_parameter(
                            reader,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::GuidelineSpecifiedDocumentContextParameter(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_guideline_specified_document_context_parameter(
                            reader,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        * self . state = ExchangedDocumentContextTypeDeserializerState :: BusinessProcessSpecifiedDocumentContextParameter (None) ;
                        event
                    }
                    (
                        S::BusinessProcessSpecifiedDocumentContextParameter(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"BusinessProcessSpecifiedDocumentContextParameter",
                        ) {
                            let output = < super :: DocumentContextParameterType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self
                                .handle_business_process_specified_document_context_parameter(
                                    reader,
                                    output,
                                    &mut fallback,
                                )? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::GuidelineSpecifiedDocumentContextParameter(None);
                            event
                        }
                    }
                    (
                        S::GuidelineSpecifiedDocumentContextParameter(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"GuidelineSpecifiedDocumentContextParameter",
                        ) {
                            let output = < super :: DocumentContextParameterType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_guideline_specified_document_context_parameter(
                                reader,
                                output,
                                &mut fallback,
                            )? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::ExchangedDocumentContextType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                ExchangedDocumentContextTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::ExchangedDocumentContextType {
                business_process_specified_document_context_parameter: self
                    .business_process_specified_document_context_parameter,
                guideline_specified_document_context_parameter: self
                    .guideline_specified_document_context_parameter
                    .ok_or_else(|| {
                        ErrorKind::MissingElement(
                            "GuidelineSpecifiedDocumentContextParameter".into(),
                        )
                    })?,
            })
        }
    }
    #[derive(Debug)]
    pub struct ExchangedDocumentTypeDeserializer {
        id: Option<super::IdType>,
        type_code: Option<super::DocumentCodeType>,
        issue_date_time: Option<super::DateTimeType>,
        included_note: Vec<super::NoteType>,
        state: Box<ExchangedDocumentTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ExchangedDocumentTypeDeserializerState {
        Init__,
        Id(Option<<super::IdType as WithDeserializer>::Deserializer>),
        TypeCode(Option<<super::DocumentCodeType as WithDeserializer>::Deserializer>),
        IssueDateTime(Option<<super::DateTimeType as WithDeserializer>::Deserializer>),
        IncludedNote(Option<<super::NoteType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl ExchangedDocumentTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
            }
            Ok(Self {
                id: None,
                type_code: None,
                issue_date_time: None,
                included_note: Vec::new(),
                state: Box::new(ExchangedDocumentTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: ExchangedDocumentTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use ExchangedDocumentTypeDeserializerState as S;
            match state {
                S::Id(Some(deserializer)) => self.store_id(deserializer.finish(reader)?)?,
                S::TypeCode(Some(deserializer)) => {
                    self.store_type_code(deserializer.finish(reader)?)?
                }
                S::IssueDateTime(Some(deserializer)) => {
                    self.store_issue_date_time(deserializer.finish(reader)?)?
                }
                S::IncludedNote(Some(deserializer)) => {
                    self.store_included_note(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_id(&mut self, value: super::IdType) -> Result<(), Error> {
            if self.id.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"ID")))?;
            }
            self.id = Some(value);
            Ok(())
        }
        fn store_type_code(&mut self, value: super::DocumentCodeType) -> Result<(), Error> {
            if self.type_code.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"TypeCode",
                )))?;
            }
            self.type_code = Some(value);
            Ok(())
        }
        fn store_issue_date_time(&mut self, value: super::DateTimeType) -> Result<(), Error> {
            if self.issue_date_time.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"IssueDateTime",
                )))?;
            }
            self.issue_date_time = Some(value);
            Ok(())
        }
        fn store_included_note(&mut self, value: super::NoteType) -> Result<(), Error> {
            self.included_note.push(value);
            Ok(())
        }
        fn handle_id<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::IdType>,
            fallback: &mut Option<ExchangedDocumentTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.id.is_some() {
                    fallback.get_or_insert(ExchangedDocumentTypeDeserializerState::Id(None));
                    *self.state = ExchangedDocumentTypeDeserializerState::TypeCode(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = ExchangedDocumentTypeDeserializerState::Id(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_id(data)?;
                    *self.state = ExchangedDocumentTypeDeserializerState::TypeCode(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(ExchangedDocumentTypeDeserializerState::Id(
                                Some(deserializer),
                            ));
                            *self.state = ExchangedDocumentTypeDeserializerState::TypeCode(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                ExchangedDocumentTypeDeserializerState::Id(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_type_code<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::DocumentCodeType>,
            fallback: &mut Option<ExchangedDocumentTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.type_code.is_some() {
                    fallback.get_or_insert(ExchangedDocumentTypeDeserializerState::TypeCode(None));
                    *self.state = ExchangedDocumentTypeDeserializerState::IssueDateTime(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = ExchangedDocumentTypeDeserializerState::TypeCode(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_type_code(data)?;
                    *self.state = ExchangedDocumentTypeDeserializerState::IssueDateTime(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                ExchangedDocumentTypeDeserializerState::TypeCode(Some(
                                    deserializer,
                                )),
                            );
                            *self.state =
                                ExchangedDocumentTypeDeserializerState::IssueDateTime(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = ExchangedDocumentTypeDeserializerState::TypeCode(Some(
                                deserializer,
                            ));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_issue_date_time<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::DateTimeType>,
            fallback: &mut Option<ExchangedDocumentTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.issue_date_time.is_some() {
                    fallback
                        .get_or_insert(ExchangedDocumentTypeDeserializerState::IssueDateTime(None));
                    *self.state = ExchangedDocumentTypeDeserializerState::IncludedNote(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = ExchangedDocumentTypeDeserializerState::IssueDateTime(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_issue_date_time(data)?;
                    *self.state = ExchangedDocumentTypeDeserializerState::IncludedNote(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                ExchangedDocumentTypeDeserializerState::IssueDateTime(Some(
                                    deserializer,
                                )),
                            );
                            *self.state =
                                ExchangedDocumentTypeDeserializerState::IncludedNote(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = ExchangedDocumentTypeDeserializerState::IssueDateTime(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
        fn handle_included_note<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::NoteType>,
            fallback: &mut Option<ExchangedDocumentTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(ExchangedDocumentTypeDeserializerState::IncludedNote(None));
                *self.state = ExchangedDocumentTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_included_note(data)?;
                    *self.state = ExchangedDocumentTypeDeserializerState::IncludedNote(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                ExchangedDocumentTypeDeserializerState::IncludedNote(Some(
                                    deserializer,
                                )),
                            );
                            *self.state =
                                ExchangedDocumentTypeDeserializerState::IncludedNote(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = ExchangedDocumentTypeDeserializerState::IncludedNote(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::ExchangedDocumentType> for ExchangedDocumentTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ExchangedDocumentType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ExchangedDocumentType>
        where
            R: DeserializeReader,
        {
            use ExchangedDocumentTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Id(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_id(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::TypeCode(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_type_code(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::IssueDateTime(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_issue_date_time(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::IncludedNote(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_included_note(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = ExchangedDocumentTypeDeserializerState::Id(None);
                        event
                    }
                    (S::Id(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_RAM), b"ID") {
                            let output = <super::IdType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            match self.handle_id(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::TypeCode(None);
                            event
                        }
                    }
                    (S::TypeCode(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_RAM), b"TypeCode") {
                            let output =
                                <super::DocumentCodeType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_type_code(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::IssueDateTime(None);
                            event
                        }
                    }
                    (S::IssueDateTime(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"IssueDateTime",
                        ) {
                            let output =
                                <super::DateTimeType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_issue_date_time(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::IncludedNote(None);
                            event
                        }
                    }
                    (S::IncludedNote(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"IncludedNote",
                        ) {
                            let output = <super::NoteType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            match self.handle_included_note(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::ExchangedDocumentType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                ExchangedDocumentTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::ExchangedDocumentType {
                id: self
                    .id
                    .ok_or_else(|| ErrorKind::MissingElement("ID".into()))?,
                type_code: self
                    .type_code
                    .ok_or_else(|| ErrorKind::MissingElement("TypeCode".into()))?,
                issue_date_time: self
                    .issue_date_time
                    .ok_or_else(|| ErrorKind::MissingElement("IssueDateTime".into()))?,
                included_note: self.included_note,
            })
        }
    }
    #[derive(Debug)]
    pub struct SupplyChainTradeTransactionTypeDeserializer {
        applicable_header_trade_agreement: Option<super::HeaderTradeAgreementType>,
        applicable_header_trade_delivery: Option<super::HeaderTradeDeliveryType>,
        applicable_header_trade_settlement: Option<super::HeaderTradeSettlementType>,
        state: Box<SupplyChainTradeTransactionTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum SupplyChainTradeTransactionTypeDeserializerState {
        Init__,
        ApplicableHeaderTradeAgreement(
            Option<<super::HeaderTradeAgreementType as WithDeserializer>::Deserializer>,
        ),
        ApplicableHeaderTradeDelivery(
            Option<<super::HeaderTradeDeliveryType as WithDeserializer>::Deserializer>,
        ),
        ApplicableHeaderTradeSettlement(
            Option<<super::HeaderTradeSettlementType as WithDeserializer>::Deserializer>,
        ),
        Done__,
        Unknown__,
    }
    impl SupplyChainTradeTransactionTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
            }
            Ok(Self {
                applicable_header_trade_agreement: None,
                applicable_header_trade_delivery: None,
                applicable_header_trade_settlement: None,
                state: Box::new(SupplyChainTradeTransactionTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: SupplyChainTradeTransactionTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use SupplyChainTradeTransactionTypeDeserializerState as S;
            match state {
                S::ApplicableHeaderTradeAgreement(Some(deserializer)) => {
                    self.store_applicable_header_trade_agreement(deserializer.finish(reader)?)?
                }
                S::ApplicableHeaderTradeDelivery(Some(deserializer)) => {
                    self.store_applicable_header_trade_delivery(deserializer.finish(reader)?)?
                }
                S::ApplicableHeaderTradeSettlement(Some(deserializer)) => {
                    self.store_applicable_header_trade_settlement(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_applicable_header_trade_agreement(
            &mut self,
            value: super::HeaderTradeAgreementType,
        ) -> Result<(), Error> {
            if self.applicable_header_trade_agreement.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"ApplicableHeaderTradeAgreement",
                )))?;
            }
            self.applicable_header_trade_agreement = Some(value);
            Ok(())
        }
        fn store_applicable_header_trade_delivery(
            &mut self,
            value: super::HeaderTradeDeliveryType,
        ) -> Result<(), Error> {
            if self.applicable_header_trade_delivery.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"ApplicableHeaderTradeDelivery",
                )))?;
            }
            self.applicable_header_trade_delivery = Some(value);
            Ok(())
        }
        fn store_applicable_header_trade_settlement(
            &mut self,
            value: super::HeaderTradeSettlementType,
        ) -> Result<(), Error> {
            if self.applicable_header_trade_settlement.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"ApplicableHeaderTradeSettlement",
                )))?;
            }
            self.applicable_header_trade_settlement = Some(value);
            Ok(())
        }
        fn handle_applicable_header_trade_agreement<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::HeaderTradeAgreementType>,
            fallback: &mut Option<SupplyChainTradeTransactionTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.applicable_header_trade_agreement.is_some() {
                    fallback . get_or_insert (SupplyChainTradeTransactionTypeDeserializerState :: ApplicableHeaderTradeAgreement (None)) ;
                    * self . state = SupplyChainTradeTransactionTypeDeserializerState :: ApplicableHeaderTradeDelivery (None) ;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    * self . state = SupplyChainTradeTransactionTypeDeserializerState :: ApplicableHeaderTradeAgreement (None) ;
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_applicable_header_trade_agreement(data)?;
                    * self . state = SupplyChainTradeTransactionTypeDeserializerState :: ApplicableHeaderTradeDelivery (None) ;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback . get_or_insert (SupplyChainTradeTransactionTypeDeserializerState :: ApplicableHeaderTradeAgreement (Some (deserializer))) ;
                            * self . state = SupplyChainTradeTransactionTypeDeserializerState :: ApplicableHeaderTradeDelivery (None) ;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            * self . state = SupplyChainTradeTransactionTypeDeserializerState :: ApplicableHeaderTradeAgreement (Some (deserializer)) ;
                        }
                    }
                    ret
                }
            })
        }
        fn handle_applicable_header_trade_delivery<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::HeaderTradeDeliveryType>,
            fallback: &mut Option<SupplyChainTradeTransactionTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.applicable_header_trade_delivery.is_some() {
                    fallback . get_or_insert (SupplyChainTradeTransactionTypeDeserializerState :: ApplicableHeaderTradeDelivery (None)) ;
                    * self . state = SupplyChainTradeTransactionTypeDeserializerState :: ApplicableHeaderTradeSettlement (None) ;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    * self . state = SupplyChainTradeTransactionTypeDeserializerState :: ApplicableHeaderTradeDelivery (None) ;
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_applicable_header_trade_delivery(data)?;
                    * self . state = SupplyChainTradeTransactionTypeDeserializerState :: ApplicableHeaderTradeSettlement (None) ;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback . get_or_insert (SupplyChainTradeTransactionTypeDeserializerState :: ApplicableHeaderTradeDelivery (Some (deserializer))) ;
                            * self . state = SupplyChainTradeTransactionTypeDeserializerState :: ApplicableHeaderTradeSettlement (None) ;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            * self . state = SupplyChainTradeTransactionTypeDeserializerState :: ApplicableHeaderTradeDelivery (Some (deserializer)) ;
                        }
                    }
                    ret
                }
            })
        }
        fn handle_applicable_header_trade_settlement<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::HeaderTradeSettlementType>,
            fallback: &mut Option<SupplyChainTradeTransactionTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.applicable_header_trade_settlement.is_some() {
                    fallback . get_or_insert (SupplyChainTradeTransactionTypeDeserializerState :: ApplicableHeaderTradeSettlement (None)) ;
                    *self.state = SupplyChainTradeTransactionTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    * self . state = SupplyChainTradeTransactionTypeDeserializerState :: ApplicableHeaderTradeSettlement (None) ;
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_applicable_header_trade_settlement(data)?;
                    *self.state = SupplyChainTradeTransactionTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback . get_or_insert (SupplyChainTradeTransactionTypeDeserializerState :: ApplicableHeaderTradeSettlement (Some (deserializer))) ;
                            *self.state = SupplyChainTradeTransactionTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            * self . state = SupplyChainTradeTransactionTypeDeserializerState :: ApplicableHeaderTradeSettlement (Some (deserializer)) ;
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::SupplyChainTradeTransactionType>
        for SupplyChainTradeTransactionTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SupplyChainTradeTransactionType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SupplyChainTradeTransactionType>
        where
            R: DeserializeReader,
        {
            use SupplyChainTradeTransactionTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::ApplicableHeaderTradeAgreement(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_applicable_header_trade_agreement(
                            reader,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::ApplicableHeaderTradeDelivery(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_applicable_header_trade_delivery(
                            reader,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::ApplicableHeaderTradeSettlement(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_applicable_header_trade_settlement(
                            reader,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        * self . state = SupplyChainTradeTransactionTypeDeserializerState :: ApplicableHeaderTradeAgreement (None) ;
                        event
                    }
                    (
                        S::ApplicableHeaderTradeAgreement(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"ApplicableHeaderTradeAgreement",
                        ) {
                            let output = < super :: HeaderTradeAgreementType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_applicable_header_trade_agreement(
                                reader,
                                output,
                                &mut fallback,
                            )? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::ApplicableHeaderTradeDelivery(None);
                            event
                        }
                    }
                    (
                        S::ApplicableHeaderTradeDelivery(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"ApplicableHeaderTradeDelivery",
                        ) {
                            let output = < super :: HeaderTradeDeliveryType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_applicable_header_trade_delivery(
                                reader,
                                output,
                                &mut fallback,
                            )? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::ApplicableHeaderTradeSettlement(None);
                            event
                        }
                    }
                    (
                        S::ApplicableHeaderTradeSettlement(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"ApplicableHeaderTradeSettlement",
                        ) {
                            let output = < super :: HeaderTradeSettlementType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_applicable_header_trade_settlement(
                                reader,
                                output,
                                &mut fallback,
                            )? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::SupplyChainTradeTransactionType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                SupplyChainTradeTransactionTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::SupplyChainTradeTransactionType {
                applicable_header_trade_agreement: self
                    .applicable_header_trade_agreement
                    .ok_or_else(|| {
                        ErrorKind::MissingElement("ApplicableHeaderTradeAgreement".into())
                    })?,
                applicable_header_trade_delivery: self
                    .applicable_header_trade_delivery
                    .ok_or_else(|| {
                        ErrorKind::MissingElement("ApplicableHeaderTradeDelivery".into())
                    })?,
                applicable_header_trade_settlement: self
                    .applicable_header_trade_settlement
                    .ok_or_else(|| {
                        ErrorKind::MissingElement("ApplicableHeaderTradeSettlement".into())
                    })?,
            })
        }
    }
    #[derive(Debug)]
    pub struct DocumentContextParameterTypeDeserializer {
        id: Option<super::IdType>,
        state: Box<DocumentContextParameterTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum DocumentContextParameterTypeDeserializerState {
        Init__,
        Id(Option<<super::IdType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl DocumentContextParameterTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
            }
            Ok(Self {
                id: None,
                state: Box::new(DocumentContextParameterTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: DocumentContextParameterTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use DocumentContextParameterTypeDeserializerState as S;
            match state {
                S::Id(Some(deserializer)) => self.store_id(deserializer.finish(reader)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_id(&mut self, value: super::IdType) -> Result<(), Error> {
            if self.id.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"ID")))?;
            }
            self.id = Some(value);
            Ok(())
        }
        fn handle_id<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::IdType>,
            fallback: &mut Option<DocumentContextParameterTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.id.is_some() {
                    fallback.get_or_insert(DocumentContextParameterTypeDeserializerState::Id(None));
                    *self.state = DocumentContextParameterTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = DocumentContextParameterTypeDeserializerState::Id(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_id(data)?;
                    *self.state = DocumentContextParameterTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                DocumentContextParameterTypeDeserializerState::Id(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = DocumentContextParameterTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = DocumentContextParameterTypeDeserializerState::Id(Some(
                                deserializer,
                            ));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::DocumentContextParameterType>
        for DocumentContextParameterTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DocumentContextParameterType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DocumentContextParameterType>
        where
            R: DeserializeReader,
        {
            use DocumentContextParameterTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Id(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_id(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = DocumentContextParameterTypeDeserializerState::Id(None);
                        event
                    }
                    (S::Id(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_RAM), b"ID") {
                            let output = <super::IdType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            match self.handle_id(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::DocumentContextParameterType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                DocumentContextParameterTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::DocumentContextParameterType {
                id: self
                    .id
                    .ok_or_else(|| ErrorKind::MissingElement("ID".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub struct IdTypeDeserializer {
        scheme_id: Option<String>,
        content: Option<String>,
        state: Box<IdTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum IdTypeDeserializerState {
        Init__,
        Content__(<String as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl IdTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut scheme_id: Option<String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_UDT),
                    Some(b"schemeID")
                ) {
                    reader.read_attrib(&mut scheme_id, b"schemeID", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib(attrib)?;
                }
            }
            Ok(Self {
                scheme_id: scheme_id,
                content: None,
                state: Box::new(IdTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: IdTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let IdTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: String) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
        ) -> DeserializerResult<'de, super::IdType>
        where
            R: DeserializeReader,
        {
            use IdTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::IdType> for IdTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::IdType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::IdType>
        where
            R: DeserializeReader,
        {
            use IdTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::IdType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, IdTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::IdType {
                scheme_id: self.scheme_id,
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct DocumentCodeTypeDeserializer {
        content: Option<String>,
        state: Box<DocumentCodeTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum DocumentCodeTypeDeserializerState {
        Init__,
        Content__(<String as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl DocumentCodeTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
            }
            Ok(Self {
                content: None,
                state: Box::new(DocumentCodeTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: DocumentCodeTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let DocumentCodeTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: String) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
        ) -> DeserializerResult<'de, super::DocumentCodeType>
        where
            R: DeserializeReader,
        {
            use DocumentCodeTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::DocumentCodeType> for DocumentCodeTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DocumentCodeType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DocumentCodeType>
        where
            R: DeserializeReader,
        {
            use DocumentCodeTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::DocumentCodeType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                DocumentCodeTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::DocumentCodeType {
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct DateTimeTypeDeserializer {
        content: Option<super::DateTimeTypeContent>,
        state: Box<DateTimeTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum DateTimeTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::DateTimeTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl DateTimeTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
            }
            Ok(Self {
                content: None,
                state: Box::new(DateTimeTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: DateTimeTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let DateTimeTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::DateTimeTypeContent) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::DateTimeTypeContent>,
            fallback: &mut Option<DateTimeTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = fallback
                    .take()
                    .unwrap_or(DateTimeTypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state = DateTimeTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = DateTimeTypeDeserializerState::Content__(deserializer);
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::DateTimeType> for DateTimeTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::DateTimeType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DateTimeType>
        where
            R: DeserializeReader,
        {
            use DateTimeTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output =
                            <super::DateTimeTypeContent as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::DateTimeType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, DateTimeTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::DateTimeType {
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct DateTimeTypeContentDeserializer {
        state: Box<DateTimeTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum DateTimeTypeContentDeserializerState {
        Init__,
        DateTimeString(
            Option<super::DateTimeTypeDateTimeStringType>,
            Option<<super::DateTimeTypeDateTimeStringType as WithDeserializer>::Deserializer>,
        ),
        Done__(super::DateTimeTypeContent),
        Unknown__,
    }
    impl DateTimeTypeContentDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<DateTimeTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                *self.state = fallback
                    .take()
                    .unwrap_or(DateTimeTypeContentDeserializerState::Init__);
                return Ok(ElementHandlerOutput::return_to_parent(event, false));
            };
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_UDT),
                Some(b"DateTimeString")
            ) {
                let output = < super :: DateTimeTypeDateTimeStringType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                return self.handle_date_time_string(
                    reader,
                    Default::default(),
                    output,
                    &mut *fallback,
                );
            }
            *self.state = fallback
                .take()
                .unwrap_or(DateTimeTypeContentDeserializerState::Init__);
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state<R>(
            reader: &R,
            state: DateTimeTypeContentDeserializerState,
        ) -> Result<super::DateTimeTypeContent, Error>
        where
            R: DeserializeReader,
        {
            use DateTimeTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::DateTimeString(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_date_time_string(&mut values, value)?;
                    }
                    Ok(super::DateTimeTypeContent::DateTimeString(
                        values.ok_or_else(|| ErrorKind::MissingElement("DateTimeString".into()))?,
                    ))
                }
                S::Done__(data) => Ok(data),
                S::Unknown__ => unreachable!(),
            }
        }
        fn store_date_time_string(
            values: &mut Option<super::DateTimeTypeDateTimeStringType>,
            value: super::DateTimeTypeDateTimeStringType,
        ) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"DateTimeString",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_date_time_string<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<super::DateTimeTypeDateTimeStringType>,
            output: DeserializerOutput<'de, super::DateTimeTypeDateTimeStringType>,
            fallback: &mut Option<DateTimeTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => DateTimeTypeContentDeserializerState::Init__,
                    Some(DateTimeTypeContentDeserializerState::DateTimeString(
                        _,
                        Some(deserializer),
                    )) => DateTimeTypeContentDeserializerState::DateTimeString(
                        values,
                        Some(deserializer),
                    ),
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(DateTimeTypeContentDeserializerState::DateTimeString(
                    _,
                    Some(deserializer),
                )) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_date_time_string(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_date_time_string(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        DateTimeTypeContentDeserializerState::DateTimeString(values, None),
                    )?;
                    *self.state = DateTimeTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = DateTimeTypeContentDeserializerState::DateTimeString(
                        values,
                        Some(deserializer),
                    );
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::DateTimeTypeContent> for DateTimeTypeContentDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DateTimeTypeContent>
        where
            R: DeserializeReader,
        {
            let deserializer = Self {
                state: Box::new(DateTimeTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state, DateTimeTypeContentDeserializerState::Init__) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DateTimeTypeContent>
        where
            R: DeserializeReader,
        {
            use DateTimeTypeContentDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::DateTimeString(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_date_time_string(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                reader, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(reader, event, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (S::DateTimeString(values, None), event) => {
                        let output = < super :: DateTimeTypeDateTimeStringType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                        match self.handle_date_time_string(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (s @ S::Done__(_), event) => {
                        *self.state = s;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = if matches!(&*self.state, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(reader)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(self, reader: &R) -> Result<super::DateTimeTypeContent, Error>
        where
            R: DeserializeReader,
        {
            Self::finish_state(reader, *self.state)
        }
    }
    #[derive(Debug)]
    pub struct NoteTypeDeserializer {
        content: Option<super::TextType>,
        subject_code: Option<super::CodeType>,
        state: Box<NoteTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum NoteTypeDeserializerState {
        Init__,
        Content(Option<<super::TextType as WithDeserializer>::Deserializer>),
        SubjectCode(Option<<super::CodeType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl NoteTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
            }
            Ok(Self {
                content: None,
                subject_code: None,
                state: Box::new(NoteTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: NoteTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use NoteTypeDeserializerState as S;
            match state {
                S::Content(Some(deserializer)) => {
                    self.store_content(deserializer.finish(reader)?)?
                }
                S::SubjectCode(Some(deserializer)) => {
                    self.store_subject_code(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::TextType) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Content",
                )))?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn store_subject_code(&mut self, value: super::CodeType) -> Result<(), Error> {
            if self.subject_code.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"SubjectCode",
                )))?;
            }
            self.subject_code = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::TextType>,
            fallback: &mut Option<NoteTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.content.is_some() {
                    fallback.get_or_insert(NoteTypeDeserializerState::Content(None));
                    *self.state = NoteTypeDeserializerState::SubjectCode(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = NoteTypeDeserializerState::Content(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state = NoteTypeDeserializerState::SubjectCode(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(NoteTypeDeserializerState::Content(Some(
                                deserializer,
                            )));
                            *self.state = NoteTypeDeserializerState::SubjectCode(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = NoteTypeDeserializerState::Content(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_subject_code<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::CodeType>,
            fallback: &mut Option<NoteTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(NoteTypeDeserializerState::SubjectCode(None));
                *self.state = NoteTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_subject_code(data)?;
                    *self.state = NoteTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(NoteTypeDeserializerState::SubjectCode(Some(
                                deserializer,
                            )));
                            *self.state = NoteTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                NoteTypeDeserializerState::SubjectCode(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::NoteType> for NoteTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::NoteType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::NoteType>
        where
            R: DeserializeReader,
        {
            use NoteTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Content(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::SubjectCode(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_subject_code(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = NoteTypeDeserializerState::Content(None);
                        event
                    }
                    (S::Content(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_RAM), b"Content") {
                            let output = <super::TextType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            match self.handle_content(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::SubjectCode(None);
                            event
                        }
                    }
                    (S::SubjectCode(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_RAM), b"SubjectCode")
                        {
                            let output = <super::CodeType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            match self.handle_subject_code(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::NoteType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, NoteTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::NoteType {
                content: self
                    .content
                    .ok_or_else(|| ErrorKind::MissingElement("Content".into()))?,
                subject_code: self.subject_code,
            })
        }
    }
    #[derive(Debug)]
    pub struct HeaderTradeAgreementTypeDeserializer {
        buyer_reference: Option<super::TextType>,
        seller_trade_party: Option<super::TradePartyType>,
        buyer_trade_party: Option<super::TradePartyType>,
        seller_tax_representative_trade_party: Option<super::TradePartyType>,
        buyer_order_referenced_document: Option<super::ReferencedDocumentType>,
        contract_referenced_document: Option<super::ReferencedDocumentType>,
        state: Box<HeaderTradeAgreementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum HeaderTradeAgreementTypeDeserializerState {
        Init__,
        BuyerReference(Option<<super::TextType as WithDeserializer>::Deserializer>),
        SellerTradeParty(Option<<super::TradePartyType as WithDeserializer>::Deserializer>),
        BuyerTradeParty(Option<<super::TradePartyType as WithDeserializer>::Deserializer>),
        SellerTaxRepresentativeTradeParty(
            Option<<super::TradePartyType as WithDeserializer>::Deserializer>,
        ),
        BuyerOrderReferencedDocument(
            Option<<super::ReferencedDocumentType as WithDeserializer>::Deserializer>,
        ),
        ContractReferencedDocument(
            Option<<super::ReferencedDocumentType as WithDeserializer>::Deserializer>,
        ),
        Done__,
        Unknown__,
    }
    impl HeaderTradeAgreementTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
            }
            Ok(Self {
                buyer_reference: None,
                seller_trade_party: None,
                buyer_trade_party: None,
                seller_tax_representative_trade_party: None,
                buyer_order_referenced_document: None,
                contract_referenced_document: None,
                state: Box::new(HeaderTradeAgreementTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: HeaderTradeAgreementTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use HeaderTradeAgreementTypeDeserializerState as S;
            match state {
                S::BuyerReference(Some(deserializer)) => {
                    self.store_buyer_reference(deserializer.finish(reader)?)?
                }
                S::SellerTradeParty(Some(deserializer)) => {
                    self.store_seller_trade_party(deserializer.finish(reader)?)?
                }
                S::BuyerTradeParty(Some(deserializer)) => {
                    self.store_buyer_trade_party(deserializer.finish(reader)?)?
                }
                S::SellerTaxRepresentativeTradeParty(Some(deserializer)) => {
                    self.store_seller_tax_representative_trade_party(deserializer.finish(reader)?)?
                }
                S::BuyerOrderReferencedDocument(Some(deserializer)) => {
                    self.store_buyer_order_referenced_document(deserializer.finish(reader)?)?
                }
                S::ContractReferencedDocument(Some(deserializer)) => {
                    self.store_contract_referenced_document(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_buyer_reference(&mut self, value: super::TextType) -> Result<(), Error> {
            if self.buyer_reference.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"BuyerReference",
                )))?;
            }
            self.buyer_reference = Some(value);
            Ok(())
        }
        fn store_seller_trade_party(&mut self, value: super::TradePartyType) -> Result<(), Error> {
            if self.seller_trade_party.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"SellerTradeParty",
                )))?;
            }
            self.seller_trade_party = Some(value);
            Ok(())
        }
        fn store_buyer_trade_party(&mut self, value: super::TradePartyType) -> Result<(), Error> {
            if self.buyer_trade_party.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"BuyerTradeParty",
                )))?;
            }
            self.buyer_trade_party = Some(value);
            Ok(())
        }
        fn store_seller_tax_representative_trade_party(
            &mut self,
            value: super::TradePartyType,
        ) -> Result<(), Error> {
            if self.seller_tax_representative_trade_party.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"SellerTaxRepresentativeTradeParty",
                )))?;
            }
            self.seller_tax_representative_trade_party = Some(value);
            Ok(())
        }
        fn store_buyer_order_referenced_document(
            &mut self,
            value: super::ReferencedDocumentType,
        ) -> Result<(), Error> {
            if self.buyer_order_referenced_document.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"BuyerOrderReferencedDocument",
                )))?;
            }
            self.buyer_order_referenced_document = Some(value);
            Ok(())
        }
        fn store_contract_referenced_document(
            &mut self,
            value: super::ReferencedDocumentType,
        ) -> Result<(), Error> {
            if self.contract_referenced_document.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"ContractReferencedDocument",
                )))?;
            }
            self.contract_referenced_document = Some(value);
            Ok(())
        }
        fn handle_buyer_reference<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::TextType>,
            fallback: &mut Option<HeaderTradeAgreementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(HeaderTradeAgreementTypeDeserializerState::BuyerReference(
                    None,
                ));
                *self.state = HeaderTradeAgreementTypeDeserializerState::SellerTradeParty(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_buyer_reference(data)?;
                    *self.state = HeaderTradeAgreementTypeDeserializerState::SellerTradeParty(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                HeaderTradeAgreementTypeDeserializerState::BuyerReference(Some(
                                    deserializer,
                                )),
                            );
                            *self.state =
                                HeaderTradeAgreementTypeDeserializerState::SellerTradeParty(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = HeaderTradeAgreementTypeDeserializerState::BuyerReference(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
        fn handle_seller_trade_party<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::TradePartyType>,
            fallback: &mut Option<HeaderTradeAgreementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.seller_trade_party.is_some() {
                    fallback.get_or_insert(
                        HeaderTradeAgreementTypeDeserializerState::SellerTradeParty(None),
                    );
                    *self.state = HeaderTradeAgreementTypeDeserializerState::BuyerTradeParty(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = HeaderTradeAgreementTypeDeserializerState::SellerTradeParty(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_seller_trade_party(data)?;
                    *self.state = HeaderTradeAgreementTypeDeserializerState::BuyerTradeParty(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                HeaderTradeAgreementTypeDeserializerState::SellerTradeParty(Some(
                                    deserializer,
                                )),
                            );
                            *self.state =
                                HeaderTradeAgreementTypeDeserializerState::BuyerTradeParty(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                HeaderTradeAgreementTypeDeserializerState::SellerTradeParty(Some(
                                    deserializer,
                                ));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_buyer_trade_party<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::TradePartyType>,
            fallback: &mut Option<HeaderTradeAgreementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.buyer_trade_party.is_some() {
                    fallback.get_or_insert(
                        HeaderTradeAgreementTypeDeserializerState::BuyerTradeParty(None),
                    );
                    * self . state = HeaderTradeAgreementTypeDeserializerState :: SellerTaxRepresentativeTradeParty (None) ;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = HeaderTradeAgreementTypeDeserializerState::BuyerTradeParty(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_buyer_trade_party(data)?;
                    * self . state = HeaderTradeAgreementTypeDeserializerState :: SellerTaxRepresentativeTradeParty (None) ;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                HeaderTradeAgreementTypeDeserializerState::BuyerTradeParty(Some(
                                    deserializer,
                                )),
                            );
                            * self . state = HeaderTradeAgreementTypeDeserializerState :: SellerTaxRepresentativeTradeParty (None) ;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                HeaderTradeAgreementTypeDeserializerState::BuyerTradeParty(Some(
                                    deserializer,
                                ));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_seller_tax_representative_trade_party<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::TradePartyType>,
            fallback: &mut Option<HeaderTradeAgreementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(
                    HeaderTradeAgreementTypeDeserializerState::SellerTaxRepresentativeTradeParty(
                        None,
                    ),
                );
                *self.state =
                    HeaderTradeAgreementTypeDeserializerState::BuyerOrderReferencedDocument(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_seller_tax_representative_trade_party(data)?;
                    *self.state =
                        HeaderTradeAgreementTypeDeserializerState::BuyerOrderReferencedDocument(
                            None,
                        );
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback . get_or_insert (HeaderTradeAgreementTypeDeserializerState :: SellerTaxRepresentativeTradeParty (Some (deserializer))) ;
                            * self . state = HeaderTradeAgreementTypeDeserializerState :: BuyerOrderReferencedDocument (None) ;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            * self . state = HeaderTradeAgreementTypeDeserializerState :: SellerTaxRepresentativeTradeParty (Some (deserializer)) ;
                        }
                    }
                    ret
                }
            })
        }
        fn handle_buyer_order_referenced_document<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::ReferencedDocumentType>,
            fallback: &mut Option<HeaderTradeAgreementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(
                    HeaderTradeAgreementTypeDeserializerState::BuyerOrderReferencedDocument(None),
                );
                *self.state =
                    HeaderTradeAgreementTypeDeserializerState::ContractReferencedDocument(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_buyer_order_referenced_document(data)?;
                    *self.state =
                        HeaderTradeAgreementTypeDeserializerState::ContractReferencedDocument(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback . get_or_insert (HeaderTradeAgreementTypeDeserializerState :: BuyerOrderReferencedDocument (Some (deserializer))) ;
                            * self . state = HeaderTradeAgreementTypeDeserializerState :: ContractReferencedDocument (None) ;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            * self . state = HeaderTradeAgreementTypeDeserializerState :: BuyerOrderReferencedDocument (Some (deserializer)) ;
                        }
                    }
                    ret
                }
            })
        }
        fn handle_contract_referenced_document<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::ReferencedDocumentType>,
            fallback: &mut Option<HeaderTradeAgreementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(
                    HeaderTradeAgreementTypeDeserializerState::ContractReferencedDocument(None),
                );
                *self.state = HeaderTradeAgreementTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_contract_referenced_document(data)?;
                    *self.state = HeaderTradeAgreementTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback . get_or_insert (HeaderTradeAgreementTypeDeserializerState :: ContractReferencedDocument (Some (deserializer))) ;
                            *self.state = HeaderTradeAgreementTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            * self . state = HeaderTradeAgreementTypeDeserializerState :: ContractReferencedDocument (Some (deserializer)) ;
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::HeaderTradeAgreementType>
        for HeaderTradeAgreementTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::HeaderTradeAgreementType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::HeaderTradeAgreementType>
        where
            R: DeserializeReader,
        {
            use HeaderTradeAgreementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::BuyerReference(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_buyer_reference(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::SellerTradeParty(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_seller_trade_party(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::BuyerTradeParty(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_buyer_trade_party(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::SellerTaxRepresentativeTradeParty(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_seller_tax_representative_trade_party(
                            reader,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::BuyerOrderReferencedDocument(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_buyer_order_referenced_document(
                            reader,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::ContractReferencedDocument(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_contract_referenced_document(
                            reader,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state =
                            HeaderTradeAgreementTypeDeserializerState::BuyerReference(None);
                        event
                    }
                    (S::BuyerReference(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"BuyerReference",
                        ) {
                            let output = <super::TextType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            match self.handle_buyer_reference(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::SellerTradeParty(None);
                            event
                        }
                    }
                    (S::SellerTradeParty(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"SellerTradeParty",
                        ) {
                            let output =
                                <super::TradePartyType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_seller_trade_party(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::BuyerTradeParty(None);
                            event
                        }
                    }
                    (S::BuyerTradeParty(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"BuyerTradeParty",
                        ) {
                            let output =
                                <super::TradePartyType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_buyer_trade_party(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::SellerTaxRepresentativeTradeParty(None);
                            event
                        }
                    }
                    (
                        S::SellerTaxRepresentativeTradeParty(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"SellerTaxRepresentativeTradeParty",
                        ) {
                            let output =
                                <super::TradePartyType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_seller_tax_representative_trade_party(
                                reader,
                                output,
                                &mut fallback,
                            )? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::BuyerOrderReferencedDocument(None);
                            event
                        }
                    }
                    (
                        S::BuyerOrderReferencedDocument(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"BuyerOrderReferencedDocument",
                        ) {
                            let output = < super :: ReferencedDocumentType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_buyer_order_referenced_document(
                                reader,
                                output,
                                &mut fallback,
                            )? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::ContractReferencedDocument(None);
                            event
                        }
                    }
                    (
                        S::ContractReferencedDocument(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"ContractReferencedDocument",
                        ) {
                            let output = < super :: ReferencedDocumentType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_contract_referenced_document(
                                reader,
                                output,
                                &mut fallback,
                            )? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::HeaderTradeAgreementType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                HeaderTradeAgreementTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::HeaderTradeAgreementType {
                buyer_reference: self.buyer_reference,
                seller_trade_party: self
                    .seller_trade_party
                    .ok_or_else(|| ErrorKind::MissingElement("SellerTradeParty".into()))?,
                buyer_trade_party: self
                    .buyer_trade_party
                    .ok_or_else(|| ErrorKind::MissingElement("BuyerTradeParty".into()))?,
                seller_tax_representative_trade_party: self.seller_tax_representative_trade_party,
                buyer_order_referenced_document: self.buyer_order_referenced_document,
                contract_referenced_document: self.contract_referenced_document,
            })
        }
    }
    #[derive(Debug)]
    pub struct HeaderTradeDeliveryTypeDeserializer {
        ship_to_trade_party: Option<super::TradePartyType>,
        actual_delivery_supply_chain_event: Option<super::SupplyChainEventType>,
        despatch_advice_referenced_document: Option<super::ReferencedDocumentType>,
        state: Box<HeaderTradeDeliveryTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum HeaderTradeDeliveryTypeDeserializerState {
        Init__,
        ShipToTradeParty(Option<<super::TradePartyType as WithDeserializer>::Deserializer>),
        ActualDeliverySupplyChainEvent(
            Option<<super::SupplyChainEventType as WithDeserializer>::Deserializer>,
        ),
        DespatchAdviceReferencedDocument(
            Option<<super::ReferencedDocumentType as WithDeserializer>::Deserializer>,
        ),
        Done__,
        Unknown__,
    }
    impl HeaderTradeDeliveryTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
            }
            Ok(Self {
                ship_to_trade_party: None,
                actual_delivery_supply_chain_event: None,
                despatch_advice_referenced_document: None,
                state: Box::new(HeaderTradeDeliveryTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: HeaderTradeDeliveryTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use HeaderTradeDeliveryTypeDeserializerState as S;
            match state {
                S::ShipToTradeParty(Some(deserializer)) => {
                    self.store_ship_to_trade_party(deserializer.finish(reader)?)?
                }
                S::ActualDeliverySupplyChainEvent(Some(deserializer)) => {
                    self.store_actual_delivery_supply_chain_event(deserializer.finish(reader)?)?
                }
                S::DespatchAdviceReferencedDocument(Some(deserializer)) => {
                    self.store_despatch_advice_referenced_document(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_ship_to_trade_party(&mut self, value: super::TradePartyType) -> Result<(), Error> {
            if self.ship_to_trade_party.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"ShipToTradeParty",
                )))?;
            }
            self.ship_to_trade_party = Some(value);
            Ok(())
        }
        fn store_actual_delivery_supply_chain_event(
            &mut self,
            value: super::SupplyChainEventType,
        ) -> Result<(), Error> {
            if self.actual_delivery_supply_chain_event.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"ActualDeliverySupplyChainEvent",
                )))?;
            }
            self.actual_delivery_supply_chain_event = Some(value);
            Ok(())
        }
        fn store_despatch_advice_referenced_document(
            &mut self,
            value: super::ReferencedDocumentType,
        ) -> Result<(), Error> {
            if self.despatch_advice_referenced_document.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"DespatchAdviceReferencedDocument",
                )))?;
            }
            self.despatch_advice_referenced_document = Some(value);
            Ok(())
        }
        fn handle_ship_to_trade_party<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::TradePartyType>,
            fallback: &mut Option<HeaderTradeDeliveryTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(HeaderTradeDeliveryTypeDeserializerState::ShipToTradeParty(
                    None,
                ));
                *self.state =
                    HeaderTradeDeliveryTypeDeserializerState::ActualDeliverySupplyChainEvent(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_ship_to_trade_party(data)?;
                    *self.state =
                        HeaderTradeDeliveryTypeDeserializerState::ActualDeliverySupplyChainEvent(
                            None,
                        );
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                HeaderTradeDeliveryTypeDeserializerState::ShipToTradeParty(Some(
                                    deserializer,
                                )),
                            );
                            * self . state = HeaderTradeDeliveryTypeDeserializerState :: ActualDeliverySupplyChainEvent (None) ;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                HeaderTradeDeliveryTypeDeserializerState::ShipToTradeParty(Some(
                                    deserializer,
                                ));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_actual_delivery_supply_chain_event<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::SupplyChainEventType>,
            fallback: &mut Option<HeaderTradeDeliveryTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(
                    HeaderTradeDeliveryTypeDeserializerState::ActualDeliverySupplyChainEvent(None),
                );
                *self.state =
                    HeaderTradeDeliveryTypeDeserializerState::DespatchAdviceReferencedDocument(
                        None,
                    );
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_actual_delivery_supply_chain_event(data)?;
                    *self.state =
                        HeaderTradeDeliveryTypeDeserializerState::DespatchAdviceReferencedDocument(
                            None,
                        );
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback . get_or_insert (HeaderTradeDeliveryTypeDeserializerState :: ActualDeliverySupplyChainEvent (Some (deserializer))) ;
                            * self . state = HeaderTradeDeliveryTypeDeserializerState :: DespatchAdviceReferencedDocument (None) ;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            * self . state = HeaderTradeDeliveryTypeDeserializerState :: ActualDeliverySupplyChainEvent (Some (deserializer)) ;
                        }
                    }
                    ret
                }
            })
        }
        fn handle_despatch_advice_referenced_document<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::ReferencedDocumentType>,
            fallback: &mut Option<HeaderTradeDeliveryTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(
                    HeaderTradeDeliveryTypeDeserializerState::DespatchAdviceReferencedDocument(
                        None,
                    ),
                );
                *self.state = HeaderTradeDeliveryTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_despatch_advice_referenced_document(data)?;
                    *self.state = HeaderTradeDeliveryTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback . get_or_insert (HeaderTradeDeliveryTypeDeserializerState :: DespatchAdviceReferencedDocument (Some (deserializer))) ;
                            *self.state = HeaderTradeDeliveryTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            * self . state = HeaderTradeDeliveryTypeDeserializerState :: DespatchAdviceReferencedDocument (Some (deserializer)) ;
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::HeaderTradeDeliveryType>
        for HeaderTradeDeliveryTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::HeaderTradeDeliveryType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::HeaderTradeDeliveryType>
        where
            R: DeserializeReader,
        {
            use HeaderTradeDeliveryTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::ShipToTradeParty(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_ship_to_trade_party(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::ActualDeliverySupplyChainEvent(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_actual_delivery_supply_chain_event(
                            reader,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::DespatchAdviceReferencedDocument(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_despatch_advice_referenced_document(
                            reader,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state =
                            HeaderTradeDeliveryTypeDeserializerState::ShipToTradeParty(None);
                        event
                    }
                    (S::ShipToTradeParty(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"ShipToTradeParty",
                        ) {
                            let output =
                                <super::TradePartyType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_ship_to_trade_party(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::ActualDeliverySupplyChainEvent(None);
                            event
                        }
                    }
                    (
                        S::ActualDeliverySupplyChainEvent(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"ActualDeliverySupplyChainEvent",
                        ) {
                            let output = < super :: SupplyChainEventType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_actual_delivery_supply_chain_event(
                                reader,
                                output,
                                &mut fallback,
                            )? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::DespatchAdviceReferencedDocument(None);
                            event
                        }
                    }
                    (
                        S::DespatchAdviceReferencedDocument(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"DespatchAdviceReferencedDocument",
                        ) {
                            let output = < super :: ReferencedDocumentType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_despatch_advice_referenced_document(
                                reader,
                                output,
                                &mut fallback,
                            )? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::HeaderTradeDeliveryType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                HeaderTradeDeliveryTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::HeaderTradeDeliveryType {
                ship_to_trade_party: self.ship_to_trade_party,
                actual_delivery_supply_chain_event: self.actual_delivery_supply_chain_event,
                despatch_advice_referenced_document: self.despatch_advice_referenced_document,
            })
        }
    }
    #[derive(Debug)]
    pub struct HeaderTradeSettlementTypeDeserializer {
        creditor_reference_id: Option<super::IdType>,
        payment_reference: Option<super::TextType>,
        tax_currency_code: Option<super::CurrencyCodeType>,
        invoice_currency_code: Option<super::CurrencyCodeType>,
        payee_trade_party: Option<super::TradePartyType>,
        specified_trade_settlement_payment_means: Vec<super::TradeSettlementPaymentMeansType>,
        applicable_trade_tax: Vec<super::TradeTaxType>,
        billing_specified_period: Option<super::SpecifiedPeriodType>,
        specified_trade_allowance_charge: Vec<super::TradeAllowanceChargeType>,
        specified_trade_payment_terms: Option<super::TradePaymentTermsType>,
        specified_trade_settlement_header_monetary_summation:
            Option<super::TradeSettlementHeaderMonetarySummationType>,
        invoice_referenced_document: Vec<super::ReferencedDocumentType>,
        receivable_specified_trade_accounting_account: Option<super::TradeAccountingAccountType>,
        state: Box<HeaderTradeSettlementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum HeaderTradeSettlementTypeDeserializerState {
        Init__ , CreditorReferenceId (Option << super :: IdType as WithDeserializer > :: Deserializer >) , PaymentReference (Option << super :: TextType as WithDeserializer > :: Deserializer >) , TaxCurrencyCode (Option << super :: CurrencyCodeType as WithDeserializer > :: Deserializer >) , InvoiceCurrencyCode (Option << super :: CurrencyCodeType as WithDeserializer > :: Deserializer >) , PayeeTradeParty (Option << super :: TradePartyType as WithDeserializer > :: Deserializer >) , SpecifiedTradeSettlementPaymentMeans (Option << super :: TradeSettlementPaymentMeansType as WithDeserializer > :: Deserializer >) , ApplicableTradeTax (Option << super :: TradeTaxType as WithDeserializer > :: Deserializer >) , BillingSpecifiedPeriod (Option << super :: SpecifiedPeriodType as WithDeserializer > :: Deserializer >) , SpecifiedTradeAllowanceCharge (Option << super :: TradeAllowanceChargeType as WithDeserializer > :: Deserializer >) , SpecifiedTradePaymentTerms (Option << super :: TradePaymentTermsType as WithDeserializer > :: Deserializer >) , SpecifiedTradeSettlementHeaderMonetarySummation (Option << super :: TradeSettlementHeaderMonetarySummationType as WithDeserializer > :: Deserializer >) , InvoiceReferencedDocument (Option << super :: ReferencedDocumentType as WithDeserializer > :: Deserializer >) , ReceivableSpecifiedTradeAccountingAccount (Option << super :: TradeAccountingAccountType as WithDeserializer > :: Deserializer >) , Done__ , Unknown__ , }
    impl HeaderTradeSettlementTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
            }
            Ok(Self {
                creditor_reference_id: None,
                payment_reference: None,
                tax_currency_code: None,
                invoice_currency_code: None,
                payee_trade_party: None,
                specified_trade_settlement_payment_means: Vec::new(),
                applicable_trade_tax: Vec::new(),
                billing_specified_period: None,
                specified_trade_allowance_charge: Vec::new(),
                specified_trade_payment_terms: None,
                specified_trade_settlement_header_monetary_summation: None,
                invoice_referenced_document: Vec::new(),
                receivable_specified_trade_accounting_account: None,
                state: Box::new(HeaderTradeSettlementTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: HeaderTradeSettlementTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use HeaderTradeSettlementTypeDeserializerState as S;
            match state {
                S::CreditorReferenceId(Some(deserializer)) => {
                    self.store_creditor_reference_id(deserializer.finish(reader)?)?
                }
                S::PaymentReference(Some(deserializer)) => {
                    self.store_payment_reference(deserializer.finish(reader)?)?
                }
                S::TaxCurrencyCode(Some(deserializer)) => {
                    self.store_tax_currency_code(deserializer.finish(reader)?)?
                }
                S::InvoiceCurrencyCode(Some(deserializer)) => {
                    self.store_invoice_currency_code(deserializer.finish(reader)?)?
                }
                S::PayeeTradeParty(Some(deserializer)) => {
                    self.store_payee_trade_party(deserializer.finish(reader)?)?
                }
                S::SpecifiedTradeSettlementPaymentMeans(Some(deserializer)) => self
                    .store_specified_trade_settlement_payment_means(deserializer.finish(reader)?)?,
                S::ApplicableTradeTax(Some(deserializer)) => {
                    self.store_applicable_trade_tax(deserializer.finish(reader)?)?
                }
                S::BillingSpecifiedPeriod(Some(deserializer)) => {
                    self.store_billing_specified_period(deserializer.finish(reader)?)?
                }
                S::SpecifiedTradeAllowanceCharge(Some(deserializer)) => {
                    self.store_specified_trade_allowance_charge(deserializer.finish(reader)?)?
                }
                S::SpecifiedTradePaymentTerms(Some(deserializer)) => {
                    self.store_specified_trade_payment_terms(deserializer.finish(reader)?)?
                }
                S::SpecifiedTradeSettlementHeaderMonetarySummation(Some(deserializer)) => self
                    .store_specified_trade_settlement_header_monetary_summation(
                        deserializer.finish(reader)?,
                    )?,
                S::InvoiceReferencedDocument(Some(deserializer)) => {
                    self.store_invoice_referenced_document(deserializer.finish(reader)?)?
                }
                S::ReceivableSpecifiedTradeAccountingAccount(Some(deserializer)) => self
                    .store_receivable_specified_trade_accounting_account(
                        deserializer.finish(reader)?,
                    )?,
                _ => (),
            }
            Ok(())
        }
        fn store_creditor_reference_id(&mut self, value: super::IdType) -> Result<(), Error> {
            if self.creditor_reference_id.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"CreditorReferenceID",
                )))?;
            }
            self.creditor_reference_id = Some(value);
            Ok(())
        }
        fn store_payment_reference(&mut self, value: super::TextType) -> Result<(), Error> {
            if self.payment_reference.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"PaymentReference",
                )))?;
            }
            self.payment_reference = Some(value);
            Ok(())
        }
        fn store_tax_currency_code(&mut self, value: super::CurrencyCodeType) -> Result<(), Error> {
            if self.tax_currency_code.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"TaxCurrencyCode",
                )))?;
            }
            self.tax_currency_code = Some(value);
            Ok(())
        }
        fn store_invoice_currency_code(
            &mut self,
            value: super::CurrencyCodeType,
        ) -> Result<(), Error> {
            if self.invoice_currency_code.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"InvoiceCurrencyCode",
                )))?;
            }
            self.invoice_currency_code = Some(value);
            Ok(())
        }
        fn store_payee_trade_party(&mut self, value: super::TradePartyType) -> Result<(), Error> {
            if self.payee_trade_party.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"PayeeTradeParty",
                )))?;
            }
            self.payee_trade_party = Some(value);
            Ok(())
        }
        fn store_specified_trade_settlement_payment_means(
            &mut self,
            value: super::TradeSettlementPaymentMeansType,
        ) -> Result<(), Error> {
            self.specified_trade_settlement_payment_means.push(value);
            Ok(())
        }
        fn store_applicable_trade_tax(&mut self, value: super::TradeTaxType) -> Result<(), Error> {
            self.applicable_trade_tax.push(value);
            Ok(())
        }
        fn store_billing_specified_period(
            &mut self,
            value: super::SpecifiedPeriodType,
        ) -> Result<(), Error> {
            if self.billing_specified_period.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"BillingSpecifiedPeriod",
                )))?;
            }
            self.billing_specified_period = Some(value);
            Ok(())
        }
        fn store_specified_trade_allowance_charge(
            &mut self,
            value: super::TradeAllowanceChargeType,
        ) -> Result<(), Error> {
            self.specified_trade_allowance_charge.push(value);
            Ok(())
        }
        fn store_specified_trade_payment_terms(
            &mut self,
            value: super::TradePaymentTermsType,
        ) -> Result<(), Error> {
            if self.specified_trade_payment_terms.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"SpecifiedTradePaymentTerms",
                )))?;
            }
            self.specified_trade_payment_terms = Some(value);
            Ok(())
        }
        fn store_specified_trade_settlement_header_monetary_summation(
            &mut self,
            value: super::TradeSettlementHeaderMonetarySummationType,
        ) -> Result<(), Error> {
            if self
                .specified_trade_settlement_header_monetary_summation
                .is_some()
            {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"SpecifiedTradeSettlementHeaderMonetarySummation",
                )))?;
            }
            self.specified_trade_settlement_header_monetary_summation = Some(value);
            Ok(())
        }
        fn store_invoice_referenced_document(
            &mut self,
            value: super::ReferencedDocumentType,
        ) -> Result<(), Error> {
            self.invoice_referenced_document.push(value);
            Ok(())
        }
        fn store_receivable_specified_trade_accounting_account(
            &mut self,
            value: super::TradeAccountingAccountType,
        ) -> Result<(), Error> {
            if self.receivable_specified_trade_accounting_account.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"ReceivableSpecifiedTradeAccountingAccount",
                )))?;
            }
            self.receivable_specified_trade_accounting_account = Some(value);
            Ok(())
        }
        fn handle_creditor_reference_id<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::IdType>,
            fallback: &mut Option<HeaderTradeSettlementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(
                    HeaderTradeSettlementTypeDeserializerState::CreditorReferenceId(None),
                );
                *self.state = HeaderTradeSettlementTypeDeserializerState::PaymentReference(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_creditor_reference_id(data)?;
                    *self.state =
                        HeaderTradeSettlementTypeDeserializerState::PaymentReference(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                HeaderTradeSettlementTypeDeserializerState::CreditorReferenceId(
                                    Some(deserializer),
                                ),
                            );
                            *self.state =
                                HeaderTradeSettlementTypeDeserializerState::PaymentReference(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                HeaderTradeSettlementTypeDeserializerState::CreditorReferenceId(
                                    Some(deserializer),
                                );
                        }
                    }
                    ret
                }
            })
        }
        fn handle_payment_reference<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::TextType>,
            fallback: &mut Option<HeaderTradeSettlementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(
                    HeaderTradeSettlementTypeDeserializerState::PaymentReference(None),
                );
                *self.state = HeaderTradeSettlementTypeDeserializerState::TaxCurrencyCode(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_payment_reference(data)?;
                    *self.state = HeaderTradeSettlementTypeDeserializerState::TaxCurrencyCode(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                HeaderTradeSettlementTypeDeserializerState::PaymentReference(Some(
                                    deserializer,
                                )),
                            );
                            *self.state =
                                HeaderTradeSettlementTypeDeserializerState::TaxCurrencyCode(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                HeaderTradeSettlementTypeDeserializerState::PaymentReference(Some(
                                    deserializer,
                                ));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_tax_currency_code<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::CurrencyCodeType>,
            fallback: &mut Option<HeaderTradeSettlementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(
                    HeaderTradeSettlementTypeDeserializerState::TaxCurrencyCode(None),
                );
                *self.state = HeaderTradeSettlementTypeDeserializerState::InvoiceCurrencyCode(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_tax_currency_code(data)?;
                    *self.state =
                        HeaderTradeSettlementTypeDeserializerState::InvoiceCurrencyCode(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                HeaderTradeSettlementTypeDeserializerState::TaxCurrencyCode(Some(
                                    deserializer,
                                )),
                            );
                            *self.state =
                                HeaderTradeSettlementTypeDeserializerState::InvoiceCurrencyCode(
                                    None,
                                );
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                HeaderTradeSettlementTypeDeserializerState::TaxCurrencyCode(Some(
                                    deserializer,
                                ));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_invoice_currency_code<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::CurrencyCodeType>,
            fallback: &mut Option<HeaderTradeSettlementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.invoice_currency_code.is_some() {
                    fallback.get_or_insert(
                        HeaderTradeSettlementTypeDeserializerState::InvoiceCurrencyCode(None),
                    );
                    *self.state = HeaderTradeSettlementTypeDeserializerState::PayeeTradeParty(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state =
                        HeaderTradeSettlementTypeDeserializerState::InvoiceCurrencyCode(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_invoice_currency_code(data)?;
                    *self.state = HeaderTradeSettlementTypeDeserializerState::PayeeTradeParty(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                HeaderTradeSettlementTypeDeserializerState::InvoiceCurrencyCode(
                                    Some(deserializer),
                                ),
                            );
                            *self.state =
                                HeaderTradeSettlementTypeDeserializerState::PayeeTradeParty(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                HeaderTradeSettlementTypeDeserializerState::InvoiceCurrencyCode(
                                    Some(deserializer),
                                );
                        }
                    }
                    ret
                }
            })
        }
        fn handle_payee_trade_party<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::TradePartyType>,
            fallback: &mut Option<HeaderTradeSettlementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(
                    HeaderTradeSettlementTypeDeserializerState::PayeeTradeParty(None),
                );
                * self . state = HeaderTradeSettlementTypeDeserializerState :: SpecifiedTradeSettlementPaymentMeans (None) ;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_payee_trade_party(data)?;
                    * self . state = HeaderTradeSettlementTypeDeserializerState :: SpecifiedTradeSettlementPaymentMeans (None) ;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                HeaderTradeSettlementTypeDeserializerState::PayeeTradeParty(Some(
                                    deserializer,
                                )),
                            );
                            * self . state = HeaderTradeSettlementTypeDeserializerState :: SpecifiedTradeSettlementPaymentMeans (None) ;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                HeaderTradeSettlementTypeDeserializerState::PayeeTradeParty(Some(
                                    deserializer,
                                ));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_specified_trade_settlement_payment_means<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::TradeSettlementPaymentMeansType>,
            fallback: &mut Option<HeaderTradeSettlementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback . get_or_insert (HeaderTradeSettlementTypeDeserializerState :: SpecifiedTradeSettlementPaymentMeans (None)) ;
                *self.state = HeaderTradeSettlementTypeDeserializerState::ApplicableTradeTax(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_specified_trade_settlement_payment_means(data)?;
                    * self . state = HeaderTradeSettlementTypeDeserializerState :: SpecifiedTradeSettlementPaymentMeans (None) ;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback . get_or_insert (HeaderTradeSettlementTypeDeserializerState :: SpecifiedTradeSettlementPaymentMeans (Some (deserializer))) ;
                            * self . state = HeaderTradeSettlementTypeDeserializerState :: SpecifiedTradeSettlementPaymentMeans (None) ;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            * self . state = HeaderTradeSettlementTypeDeserializerState :: SpecifiedTradeSettlementPaymentMeans (Some (deserializer)) ;
                        }
                    }
                    ret
                }
            })
        }
        fn handle_applicable_trade_tax<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::TradeTaxType>,
            fallback: &mut Option<HeaderTradeSettlementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.applicable_trade_tax.len() < 1usize {
                    *self.state =
                        HeaderTradeSettlementTypeDeserializerState::ApplicableTradeTax(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                } else {
                    fallback.get_or_insert(
                        HeaderTradeSettlementTypeDeserializerState::ApplicableTradeTax(None),
                    );
                    *self.state =
                        HeaderTradeSettlementTypeDeserializerState::BillingSpecifiedPeriod(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_applicable_trade_tax(data)?;
                    *self.state =
                        HeaderTradeSettlementTypeDeserializerState::ApplicableTradeTax(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                HeaderTradeSettlementTypeDeserializerState::ApplicableTradeTax(
                                    Some(deserializer),
                                ),
                            );
                            if self.applicable_trade_tax.len().saturating_add(1) < 1usize {
                                *self.state =
                                    HeaderTradeSettlementTypeDeserializerState::ApplicableTradeTax(
                                        None,
                                    );
                            } else {
                                * self . state = HeaderTradeSettlementTypeDeserializerState :: BillingSpecifiedPeriod (None) ;
                            }
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                HeaderTradeSettlementTypeDeserializerState::ApplicableTradeTax(
                                    Some(deserializer),
                                );
                        }
                    }
                    ret
                }
            })
        }
        fn handle_billing_specified_period<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::SpecifiedPeriodType>,
            fallback: &mut Option<HeaderTradeSettlementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(
                    HeaderTradeSettlementTypeDeserializerState::BillingSpecifiedPeriod(None),
                );
                *self.state =
                    HeaderTradeSettlementTypeDeserializerState::SpecifiedTradeAllowanceCharge(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_billing_specified_period(data)?;
                    *self.state =
                        HeaderTradeSettlementTypeDeserializerState::SpecifiedTradeAllowanceCharge(
                            None,
                        );
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                HeaderTradeSettlementTypeDeserializerState::BillingSpecifiedPeriod(
                                    Some(deserializer),
                                ),
                            );
                            * self . state = HeaderTradeSettlementTypeDeserializerState :: SpecifiedTradeAllowanceCharge (None) ;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                HeaderTradeSettlementTypeDeserializerState::BillingSpecifiedPeriod(
                                    Some(deserializer),
                                );
                        }
                    }
                    ret
                }
            })
        }
        fn handle_specified_trade_allowance_charge<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::TradeAllowanceChargeType>,
            fallback: &mut Option<HeaderTradeSettlementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(
                    HeaderTradeSettlementTypeDeserializerState::SpecifiedTradeAllowanceCharge(None),
                );
                *self.state =
                    HeaderTradeSettlementTypeDeserializerState::SpecifiedTradePaymentTerms(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_specified_trade_allowance_charge(data)?;
                    *self.state =
                        HeaderTradeSettlementTypeDeserializerState::SpecifiedTradeAllowanceCharge(
                            None,
                        );
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback . get_or_insert (HeaderTradeSettlementTypeDeserializerState :: SpecifiedTradeAllowanceCharge (Some (deserializer))) ;
                            * self . state = HeaderTradeSettlementTypeDeserializerState :: SpecifiedTradeAllowanceCharge (None) ;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            * self . state = HeaderTradeSettlementTypeDeserializerState :: SpecifiedTradeAllowanceCharge (Some (deserializer)) ;
                        }
                    }
                    ret
                }
            })
        }
        fn handle_specified_trade_payment_terms<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::TradePaymentTermsType>,
            fallback: &mut Option<HeaderTradeSettlementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(
                    HeaderTradeSettlementTypeDeserializerState::SpecifiedTradePaymentTerms(None),
                );
                * self . state = HeaderTradeSettlementTypeDeserializerState :: SpecifiedTradeSettlementHeaderMonetarySummation (None) ;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_specified_trade_payment_terms(data)?;
                    * self . state = HeaderTradeSettlementTypeDeserializerState :: SpecifiedTradeSettlementHeaderMonetarySummation (None) ;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback . get_or_insert (HeaderTradeSettlementTypeDeserializerState :: SpecifiedTradePaymentTerms (Some (deserializer))) ;
                            * self . state = HeaderTradeSettlementTypeDeserializerState :: SpecifiedTradeSettlementHeaderMonetarySummation (None) ;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            * self . state = HeaderTradeSettlementTypeDeserializerState :: SpecifiedTradePaymentTerms (Some (deserializer)) ;
                        }
                    }
                    ret
                }
            })
        }
        fn handle_specified_trade_settlement_header_monetary_summation<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::TradeSettlementHeaderMonetarySummationType>,
            fallback: &mut Option<HeaderTradeSettlementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self
                    .specified_trade_settlement_header_monetary_summation
                    .is_some()
                {
                    fallback . get_or_insert (HeaderTradeSettlementTypeDeserializerState :: SpecifiedTradeSettlementHeaderMonetarySummation (None)) ;
                    *self.state =
                        HeaderTradeSettlementTypeDeserializerState::InvoiceReferencedDocument(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    * self . state = HeaderTradeSettlementTypeDeserializerState :: SpecifiedTradeSettlementHeaderMonetarySummation (None) ;
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_specified_trade_settlement_header_monetary_summation(data)?;
                    *self.state =
                        HeaderTradeSettlementTypeDeserializerState::InvoiceReferencedDocument(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback . get_or_insert (HeaderTradeSettlementTypeDeserializerState :: SpecifiedTradeSettlementHeaderMonetarySummation (Some (deserializer))) ;
                            * self . state = HeaderTradeSettlementTypeDeserializerState :: InvoiceReferencedDocument (None) ;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            * self . state = HeaderTradeSettlementTypeDeserializerState :: SpecifiedTradeSettlementHeaderMonetarySummation (Some (deserializer)) ;
                        }
                    }
                    ret
                }
            })
        }
        fn handle_invoice_referenced_document<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::ReferencedDocumentType>,
            fallback: &mut Option<HeaderTradeSettlementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(
                    HeaderTradeSettlementTypeDeserializerState::InvoiceReferencedDocument(None),
                );
                * self . state = HeaderTradeSettlementTypeDeserializerState :: ReceivableSpecifiedTradeAccountingAccount (None) ;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_invoice_referenced_document(data)?;
                    *self.state =
                        HeaderTradeSettlementTypeDeserializerState::InvoiceReferencedDocument(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback . get_or_insert (HeaderTradeSettlementTypeDeserializerState :: InvoiceReferencedDocument (Some (deserializer))) ;
                            * self . state = HeaderTradeSettlementTypeDeserializerState :: InvoiceReferencedDocument (None) ;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            * self . state = HeaderTradeSettlementTypeDeserializerState :: InvoiceReferencedDocument (Some (deserializer)) ;
                        }
                    }
                    ret
                }
            })
        }
        fn handle_receivable_specified_trade_accounting_account<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::TradeAccountingAccountType>,
            fallback: &mut Option<HeaderTradeSettlementTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback . get_or_insert (HeaderTradeSettlementTypeDeserializerState :: ReceivableSpecifiedTradeAccountingAccount (None)) ;
                *self.state = HeaderTradeSettlementTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_receivable_specified_trade_accounting_account(data)?;
                    *self.state = HeaderTradeSettlementTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback . get_or_insert (HeaderTradeSettlementTypeDeserializerState :: ReceivableSpecifiedTradeAccountingAccount (Some (deserializer))) ;
                            *self.state = HeaderTradeSettlementTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            * self . state = HeaderTradeSettlementTypeDeserializerState :: ReceivableSpecifiedTradeAccountingAccount (Some (deserializer)) ;
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::HeaderTradeSettlementType>
        for HeaderTradeSettlementTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::HeaderTradeSettlementType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::HeaderTradeSettlementType>
        where
            R: DeserializeReader,
        {
            use HeaderTradeSettlementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::CreditorReferenceId(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_creditor_reference_id(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::PaymentReference(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_payment_reference(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::TaxCurrencyCode(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_tax_currency_code(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::InvoiceCurrencyCode(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_invoice_currency_code(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::PayeeTradeParty(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_payee_trade_party(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::SpecifiedTradeSettlementPaymentMeans(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_specified_trade_settlement_payment_means(
                            reader,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::ApplicableTradeTax(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_applicable_trade_tax(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::BillingSpecifiedPeriod(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_billing_specified_period(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::SpecifiedTradeAllowanceCharge(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_specified_trade_allowance_charge(
                            reader,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::SpecifiedTradePaymentTerms(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_specified_trade_payment_terms(
                            reader,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (
                        S::SpecifiedTradeSettlementHeaderMonetarySummation(Some(deserializer)),
                        event,
                    ) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_specified_trade_settlement_header_monetary_summation(
                            reader,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::InvoiceReferencedDocument(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_invoice_referenced_document(
                            reader,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::ReceivableSpecifiedTradeAccountingAccount(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_receivable_specified_trade_accounting_account(
                            reader,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state =
                            HeaderTradeSettlementTypeDeserializerState::CreditorReferenceId(None);
                        event
                    }
                    (S::CreditorReferenceId(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"CreditorReferenceID",
                        ) {
                            let output = <super::IdType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            match self.handle_creditor_reference_id(
                                reader,
                                output,
                                &mut fallback,
                            )? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::PaymentReference(None);
                            event
                        }
                    }
                    (S::PaymentReference(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"PaymentReference",
                        ) {
                            let output = <super::TextType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            match self.handle_payment_reference(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::TaxCurrencyCode(None);
                            event
                        }
                    }
                    (S::TaxCurrencyCode(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"TaxCurrencyCode",
                        ) {
                            let output =
                                <super::CurrencyCodeType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_tax_currency_code(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::InvoiceCurrencyCode(None);
                            event
                        }
                    }
                    (S::InvoiceCurrencyCode(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"InvoiceCurrencyCode",
                        ) {
                            let output =
                                <super::CurrencyCodeType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_invoice_currency_code(
                                reader,
                                output,
                                &mut fallback,
                            )? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::PayeeTradeParty(None);
                            event
                        }
                    }
                    (S::PayeeTradeParty(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"PayeeTradeParty",
                        ) {
                            let output =
                                <super::TradePartyType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_payee_trade_party(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::SpecifiedTradeSettlementPaymentMeans(None);
                            event
                        }
                    }
                    (
                        S::SpecifiedTradeSettlementPaymentMeans(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"SpecifiedTradeSettlementPaymentMeans",
                        ) {
                            let output = < super :: TradeSettlementPaymentMeansType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_specified_trade_settlement_payment_means(
                                reader,
                                output,
                                &mut fallback,
                            )? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::ApplicableTradeTax(None);
                            event
                        }
                    }
                    (S::ApplicableTradeTax(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"ApplicableTradeTax",
                        ) {
                            let output =
                                <super::TradeTaxType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_applicable_trade_tax(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::BillingSpecifiedPeriod(None);
                            event
                        }
                    }
                    (
                        S::BillingSpecifiedPeriod(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"BillingSpecifiedPeriod",
                        ) {
                            let output = < super :: SpecifiedPeriodType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_billing_specified_period(
                                reader,
                                output,
                                &mut fallback,
                            )? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::SpecifiedTradeAllowanceCharge(None);
                            event
                        }
                    }
                    (
                        S::SpecifiedTradeAllowanceCharge(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"SpecifiedTradeAllowanceCharge",
                        ) {
                            let output = < super :: TradeAllowanceChargeType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_specified_trade_allowance_charge(
                                reader,
                                output,
                                &mut fallback,
                            )? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::SpecifiedTradePaymentTerms(None);
                            event
                        }
                    }
                    (
                        S::SpecifiedTradePaymentTerms(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"SpecifiedTradePaymentTerms",
                        ) {
                            let output = < super :: TradePaymentTermsType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_specified_trade_payment_terms(
                                reader,
                                output,
                                &mut fallback,
                            )? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::SpecifiedTradeSettlementHeaderMonetarySummation(None);
                            event
                        }
                    }
                    (
                        S::SpecifiedTradeSettlementHeaderMonetarySummation(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"SpecifiedTradeSettlementHeaderMonetarySummation",
                        ) {
                            let output = < super :: TradeSettlementHeaderMonetarySummationType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_specified_trade_settlement_header_monetary_summation(
                                reader,
                                output,
                                &mut fallback,
                            )? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::InvoiceReferencedDocument(None);
                            event
                        }
                    }
                    (
                        S::InvoiceReferencedDocument(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"InvoiceReferencedDocument",
                        ) {
                            let output = < super :: ReferencedDocumentType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_invoice_referenced_document(
                                reader,
                                output,
                                &mut fallback,
                            )? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::ReceivableSpecifiedTradeAccountingAccount(None);
                            event
                        }
                    }
                    (
                        S::ReceivableSpecifiedTradeAccountingAccount(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"ReceivableSpecifiedTradeAccountingAccount",
                        ) {
                            let output = < super :: TradeAccountingAccountType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_receivable_specified_trade_accounting_account(
                                reader,
                                output,
                                &mut fallback,
                            )? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::HeaderTradeSettlementType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                HeaderTradeSettlementTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::HeaderTradeSettlementType {
                creditor_reference_id: self.creditor_reference_id,
                payment_reference: self.payment_reference,
                tax_currency_code: self.tax_currency_code,
                invoice_currency_code: self
                    .invoice_currency_code
                    .ok_or_else(|| ErrorKind::MissingElement("InvoiceCurrencyCode".into()))?,
                payee_trade_party: self.payee_trade_party,
                specified_trade_settlement_payment_means: self
                    .specified_trade_settlement_payment_means,
                applicable_trade_tax: self.applicable_trade_tax,
                billing_specified_period: self.billing_specified_period,
                specified_trade_allowance_charge: self.specified_trade_allowance_charge,
                specified_trade_payment_terms: self.specified_trade_payment_terms,
                specified_trade_settlement_header_monetary_summation: self
                    .specified_trade_settlement_header_monetary_summation
                    .ok_or_else(|| {
                        ErrorKind::MissingElement(
                            "SpecifiedTradeSettlementHeaderMonetarySummation".into(),
                        )
                    })?,
                invoice_referenced_document: self.invoice_referenced_document,
                receivable_specified_trade_accounting_account: self
                    .receivable_specified_trade_accounting_account,
            })
        }
    }
    #[derive(Debug)]
    pub struct DateTimeTypeDateTimeStringTypeDeserializer {
        format: String,
        content: Option<String>,
        state: Box<DateTimeTypeDateTimeStringTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum DateTimeTypeDateTimeStringTypeDeserializerState {
        Init__,
        Content__(<String as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl DateTimeTypeDateTimeStringTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut format: Option<String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_UDT),
                    Some(b"format")
                ) {
                    reader.read_attrib(&mut format, b"format", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib(attrib)?;
                }
            }
            Ok(Self {
                format: format.ok_or_else(|| {
                    reader.map_error(ErrorKind::MissingAttribute("format".into()))
                })?,
                content: None,
                state: Box::new(DateTimeTypeDateTimeStringTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: DateTimeTypeDateTimeStringTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let DateTimeTypeDateTimeStringTypeDeserializerState::Content__(deserializer) = state
            {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: String) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
        ) -> DeserializerResult<'de, super::DateTimeTypeDateTimeStringType>
        where
            R: DeserializeReader,
        {
            use DateTimeTypeDateTimeStringTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::DateTimeTypeDateTimeStringType>
        for DateTimeTypeDateTimeStringTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DateTimeTypeDateTimeStringType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DateTimeTypeDateTimeStringType>
        where
            R: DeserializeReader,
        {
            use DateTimeTypeDateTimeStringTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::DateTimeTypeDateTimeStringType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                DateTimeTypeDateTimeStringTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::DateTimeTypeDateTimeStringType {
                format: self.format,
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct TextTypeDeserializer {
        content: Option<String>,
        state: Box<TextTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum TextTypeDeserializerState {
        Init__,
        Content__(<String as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl TextTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
            }
            Ok(Self {
                content: None,
                state: Box::new(TextTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: TextTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let TextTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: String) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
        ) -> DeserializerResult<'de, super::TextType>
        where
            R: DeserializeReader,
        {
            use TextTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::TextType> for TextTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::TextType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TextType>
        where
            R: DeserializeReader,
        {
            use TextTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::TextType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, TextTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::TextType {
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct CodeTypeDeserializer {
        content: Option<String>,
        state: Box<CodeTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum CodeTypeDeserializerState {
        Init__,
        Content__(<String as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl CodeTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
            }
            Ok(Self {
                content: None,
                state: Box::new(CodeTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: CodeTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let CodeTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: String) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
        ) -> DeserializerResult<'de, super::CodeType>
        where
            R: DeserializeReader,
        {
            use CodeTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::CodeType> for CodeTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::CodeType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::CodeType>
        where
            R: DeserializeReader,
        {
            use CodeTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::CodeType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, CodeTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::CodeType {
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct TradePartyTypeDeserializer {
        id: Vec<super::IdType>,
        global_id: Vec<super::IdType>,
        name: Option<super::TextType>,
        specified_legal_organization: Option<super::LegalOrganizationType>,
        postal_trade_address: Option<super::TradeAddressType>,
        uri_universal_communication: Option<super::UniversalCommunicationType>,
        specified_tax_registration: Vec<super::TaxRegistrationType>,
        state: Box<TradePartyTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum TradePartyTypeDeserializerState {
        Init__,
        Id(Option<<super::IdType as WithDeserializer>::Deserializer>),
        GlobalId(Option<<super::IdType as WithDeserializer>::Deserializer>),
        Name(Option<<super::TextType as WithDeserializer>::Deserializer>),
        SpecifiedLegalOrganization(
            Option<<super::LegalOrganizationType as WithDeserializer>::Deserializer>,
        ),
        PostalTradeAddress(Option<<super::TradeAddressType as WithDeserializer>::Deserializer>),
        UriUniversalCommunication(
            Option<<super::UniversalCommunicationType as WithDeserializer>::Deserializer>,
        ),
        SpecifiedTaxRegistration(
            Option<<super::TaxRegistrationType as WithDeserializer>::Deserializer>,
        ),
        Done__,
        Unknown__,
    }
    impl TradePartyTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
            }
            Ok(Self {
                id: Vec::new(),
                global_id: Vec::new(),
                name: None,
                specified_legal_organization: None,
                postal_trade_address: None,
                uri_universal_communication: None,
                specified_tax_registration: Vec::new(),
                state: Box::new(TradePartyTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: TradePartyTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use TradePartyTypeDeserializerState as S;
            match state {
                S::Id(Some(deserializer)) => self.store_id(deserializer.finish(reader)?)?,
                S::GlobalId(Some(deserializer)) => {
                    self.store_global_id(deserializer.finish(reader)?)?
                }
                S::Name(Some(deserializer)) => self.store_name(deserializer.finish(reader)?)?,
                S::SpecifiedLegalOrganization(Some(deserializer)) => {
                    self.store_specified_legal_organization(deserializer.finish(reader)?)?
                }
                S::PostalTradeAddress(Some(deserializer)) => {
                    self.store_postal_trade_address(deserializer.finish(reader)?)?
                }
                S::UriUniversalCommunication(Some(deserializer)) => {
                    self.store_uri_universal_communication(deserializer.finish(reader)?)?
                }
                S::SpecifiedTaxRegistration(Some(deserializer)) => {
                    self.store_specified_tax_registration(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_id(&mut self, value: super::IdType) -> Result<(), Error> {
            self.id.push(value);
            Ok(())
        }
        fn store_global_id(&mut self, value: super::IdType) -> Result<(), Error> {
            self.global_id.push(value);
            Ok(())
        }
        fn store_name(&mut self, value: super::TextType) -> Result<(), Error> {
            if self.name.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Name")))?;
            }
            self.name = Some(value);
            Ok(())
        }
        fn store_specified_legal_organization(
            &mut self,
            value: super::LegalOrganizationType,
        ) -> Result<(), Error> {
            if self.specified_legal_organization.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"SpecifiedLegalOrganization",
                )))?;
            }
            self.specified_legal_organization = Some(value);
            Ok(())
        }
        fn store_postal_trade_address(
            &mut self,
            value: super::TradeAddressType,
        ) -> Result<(), Error> {
            if self.postal_trade_address.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"PostalTradeAddress",
                )))?;
            }
            self.postal_trade_address = Some(value);
            Ok(())
        }
        fn store_uri_universal_communication(
            &mut self,
            value: super::UniversalCommunicationType,
        ) -> Result<(), Error> {
            if self.uri_universal_communication.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"URIUniversalCommunication",
                )))?;
            }
            self.uri_universal_communication = Some(value);
            Ok(())
        }
        fn store_specified_tax_registration(
            &mut self,
            value: super::TaxRegistrationType,
        ) -> Result<(), Error> {
            self.specified_tax_registration.push(value);
            Ok(())
        }
        fn handle_id<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::IdType>,
            fallback: &mut Option<TradePartyTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(TradePartyTypeDeserializerState::Id(None));
                *self.state = TradePartyTypeDeserializerState::GlobalId(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_id(data)?;
                    *self.state = TradePartyTypeDeserializerState::Id(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(TradePartyTypeDeserializerState::Id(Some(
                                deserializer,
                            )));
                            *self.state = TradePartyTypeDeserializerState::Id(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = TradePartyTypeDeserializerState::Id(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_global_id<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::IdType>,
            fallback: &mut Option<TradePartyTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(TradePartyTypeDeserializerState::GlobalId(None));
                *self.state = TradePartyTypeDeserializerState::Name(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_global_id(data)?;
                    *self.state = TradePartyTypeDeserializerState::GlobalId(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(TradePartyTypeDeserializerState::GlobalId(
                                Some(deserializer),
                            ));
                            *self.state = TradePartyTypeDeserializerState::GlobalId(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                TradePartyTypeDeserializerState::GlobalId(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_name<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::TextType>,
            fallback: &mut Option<TradePartyTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(TradePartyTypeDeserializerState::Name(None));
                *self.state = TradePartyTypeDeserializerState::SpecifiedLegalOrganization(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_name(data)?;
                    *self.state = TradePartyTypeDeserializerState::SpecifiedLegalOrganization(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(TradePartyTypeDeserializerState::Name(Some(
                                deserializer,
                            )));
                            *self.state =
                                TradePartyTypeDeserializerState::SpecifiedLegalOrganization(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = TradePartyTypeDeserializerState::Name(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_specified_legal_organization<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::LegalOrganizationType>,
            fallback: &mut Option<TradePartyTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(
                    TradePartyTypeDeserializerState::SpecifiedLegalOrganization(None),
                );
                *self.state = TradePartyTypeDeserializerState::PostalTradeAddress(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_specified_legal_organization(data)?;
                    *self.state = TradePartyTypeDeserializerState::PostalTradeAddress(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                TradePartyTypeDeserializerState::SpecifiedLegalOrganization(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = TradePartyTypeDeserializerState::PostalTradeAddress(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                TradePartyTypeDeserializerState::SpecifiedLegalOrganization(Some(
                                    deserializer,
                                ));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_postal_trade_address<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::TradeAddressType>,
            fallback: &mut Option<TradePartyTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(TradePartyTypeDeserializerState::PostalTradeAddress(None));
                *self.state = TradePartyTypeDeserializerState::UriUniversalCommunication(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_postal_trade_address(data)?;
                    *self.state = TradePartyTypeDeserializerState::UriUniversalCommunication(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                TradePartyTypeDeserializerState::PostalTradeAddress(Some(
                                    deserializer,
                                )),
                            );
                            *self.state =
                                TradePartyTypeDeserializerState::UriUniversalCommunication(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = TradePartyTypeDeserializerState::PostalTradeAddress(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
        fn handle_uri_universal_communication<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::UniversalCommunicationType>,
            fallback: &mut Option<TradePartyTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(TradePartyTypeDeserializerState::UriUniversalCommunication(
                    None,
                ));
                *self.state = TradePartyTypeDeserializerState::SpecifiedTaxRegistration(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_uri_universal_communication(data)?;
                    *self.state = TradePartyTypeDeserializerState::SpecifiedTaxRegistration(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                TradePartyTypeDeserializerState::UriUniversalCommunication(Some(
                                    deserializer,
                                )),
                            );
                            *self.state =
                                TradePartyTypeDeserializerState::SpecifiedTaxRegistration(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                TradePartyTypeDeserializerState::UriUniversalCommunication(Some(
                                    deserializer,
                                ));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_specified_tax_registration<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::TaxRegistrationType>,
            fallback: &mut Option<TradePartyTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(TradePartyTypeDeserializerState::SpecifiedTaxRegistration(
                    None,
                ));
                *self.state = TradePartyTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_specified_tax_registration(data)?;
                    if self.specified_tax_registration.len() < 2usize {
                        *self.state =
                            TradePartyTypeDeserializerState::SpecifiedTaxRegistration(None);
                    } else {
                        *self.state = TradePartyTypeDeserializerState::Done__;
                    }
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                TradePartyTypeDeserializerState::SpecifiedTaxRegistration(Some(
                                    deserializer,
                                )),
                            );
                            *self.state =
                                TradePartyTypeDeserializerState::SpecifiedTaxRegistration(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = TradePartyTypeDeserializerState::SpecifiedTaxRegistration(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::TradePartyType> for TradePartyTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::TradePartyType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TradePartyType>
        where
            R: DeserializeReader,
        {
            use TradePartyTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Id(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_id(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::GlobalId(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_global_id(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Name(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_name(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::SpecifiedLegalOrganization(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_specified_legal_organization(
                            reader,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::PostalTradeAddress(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_postal_trade_address(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::UriUniversalCommunication(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_uri_universal_communication(
                            reader,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::SpecifiedTaxRegistration(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_specified_tax_registration(
                            reader,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = TradePartyTypeDeserializerState::Id(None);
                        event
                    }
                    (S::Id(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_RAM), b"ID") {
                            let output = <super::IdType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            match self.handle_id(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::GlobalId(None);
                            event
                        }
                    }
                    (S::GlobalId(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_RAM), b"GlobalID") {
                            let output = <super::IdType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            match self.handle_global_id(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Name(None);
                            event
                        }
                    }
                    (S::Name(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_RAM), b"Name") {
                            let output = <super::TextType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            match self.handle_name(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::SpecifiedLegalOrganization(None);
                            event
                        }
                    }
                    (
                        S::SpecifiedLegalOrganization(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"SpecifiedLegalOrganization",
                        ) {
                            let output = < super :: LegalOrganizationType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_specified_legal_organization(
                                reader,
                                output,
                                &mut fallback,
                            )? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::PostalTradeAddress(None);
                            event
                        }
                    }
                    (S::PostalTradeAddress(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"PostalTradeAddress",
                        ) {
                            let output =
                                <super::TradeAddressType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_postal_trade_address(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::UriUniversalCommunication(None);
                            event
                        }
                    }
                    (
                        S::UriUniversalCommunication(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"URIUniversalCommunication",
                        ) {
                            let output = < super :: UniversalCommunicationType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_uri_universal_communication(
                                reader,
                                output,
                                &mut fallback,
                            )? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::SpecifiedTaxRegistration(None);
                            event
                        }
                    }
                    (
                        S::SpecifiedTaxRegistration(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"SpecifiedTaxRegistration",
                        ) {
                            let output = < super :: TaxRegistrationType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_specified_tax_registration(
                                reader,
                                output,
                                &mut fallback,
                            )? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::TradePartyType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, TradePartyTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::TradePartyType {
                id: self.id,
                global_id: self.global_id,
                name: self.name,
                specified_legal_organization: self.specified_legal_organization,
                postal_trade_address: self.postal_trade_address,
                uri_universal_communication: self.uri_universal_communication,
                specified_tax_registration: self.specified_tax_registration,
            })
        }
    }
    #[derive(Debug)]
    pub struct ReferencedDocumentTypeDeserializer {
        issuer_assigned_id: Option<super::IdType>,
        formatted_issue_date_time: Option<super::FormattedDateTimeType>,
        state: Box<ReferencedDocumentTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ReferencedDocumentTypeDeserializerState {
        Init__,
        IssuerAssignedId(Option<<super::IdType as WithDeserializer>::Deserializer>),
        FormattedIssueDateTime(
            Option<<super::FormattedDateTimeType as WithDeserializer>::Deserializer>,
        ),
        Done__,
        Unknown__,
    }
    impl ReferencedDocumentTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
            }
            Ok(Self {
                issuer_assigned_id: None,
                formatted_issue_date_time: None,
                state: Box::new(ReferencedDocumentTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: ReferencedDocumentTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use ReferencedDocumentTypeDeserializerState as S;
            match state {
                S::IssuerAssignedId(Some(deserializer)) => {
                    self.store_issuer_assigned_id(deserializer.finish(reader)?)?
                }
                S::FormattedIssueDateTime(Some(deserializer)) => {
                    self.store_formatted_issue_date_time(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_issuer_assigned_id(&mut self, value: super::IdType) -> Result<(), Error> {
            if self.issuer_assigned_id.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"IssuerAssignedID",
                )))?;
            }
            self.issuer_assigned_id = Some(value);
            Ok(())
        }
        fn store_formatted_issue_date_time(
            &mut self,
            value: super::FormattedDateTimeType,
        ) -> Result<(), Error> {
            if self.formatted_issue_date_time.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"FormattedIssueDateTime",
                )))?;
            }
            self.formatted_issue_date_time = Some(value);
            Ok(())
        }
        fn handle_issuer_assigned_id<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::IdType>,
            fallback: &mut Option<ReferencedDocumentTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.issuer_assigned_id.is_some() {
                    fallback.get_or_insert(
                        ReferencedDocumentTypeDeserializerState::IssuerAssignedId(None),
                    );
                    *self.state =
                        ReferencedDocumentTypeDeserializerState::FormattedIssueDateTime(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = ReferencedDocumentTypeDeserializerState::IssuerAssignedId(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_issuer_assigned_id(data)?;
                    *self.state =
                        ReferencedDocumentTypeDeserializerState::FormattedIssueDateTime(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                ReferencedDocumentTypeDeserializerState::IssuerAssignedId(Some(
                                    deserializer,
                                )),
                            );
                            *self.state =
                                ReferencedDocumentTypeDeserializerState::FormattedIssueDateTime(
                                    None,
                                );
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = ReferencedDocumentTypeDeserializerState::IssuerAssignedId(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
        fn handle_formatted_issue_date_time<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::FormattedDateTimeType>,
            fallback: &mut Option<ReferencedDocumentTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(
                    ReferencedDocumentTypeDeserializerState::FormattedIssueDateTime(None),
                );
                *self.state = ReferencedDocumentTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_formatted_issue_date_time(data)?;
                    *self.state = ReferencedDocumentTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                ReferencedDocumentTypeDeserializerState::FormattedIssueDateTime(
                                    Some(deserializer),
                                ),
                            );
                            *self.state = ReferencedDocumentTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                ReferencedDocumentTypeDeserializerState::FormattedIssueDateTime(
                                    Some(deserializer),
                                );
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::ReferencedDocumentType> for ReferencedDocumentTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ReferencedDocumentType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::ReferencedDocumentType>
        where
            R: DeserializeReader,
        {
            use ReferencedDocumentTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::IssuerAssignedId(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_issuer_assigned_id(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::FormattedIssueDateTime(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_formatted_issue_date_time(
                            reader,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state =
                            ReferencedDocumentTypeDeserializerState::IssuerAssignedId(None);
                        event
                    }
                    (S::IssuerAssignedId(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"IssuerAssignedID",
                        ) {
                            let output = <super::IdType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            match self.handle_issuer_assigned_id(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::FormattedIssueDateTime(None);
                            event
                        }
                    }
                    (
                        S::FormattedIssueDateTime(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"FormattedIssueDateTime",
                        ) {
                            let output = < super :: FormattedDateTimeType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_formatted_issue_date_time(
                                reader,
                                output,
                                &mut fallback,
                            )? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::ReferencedDocumentType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                ReferencedDocumentTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::ReferencedDocumentType {
                issuer_assigned_id: self
                    .issuer_assigned_id
                    .ok_or_else(|| ErrorKind::MissingElement("IssuerAssignedID".into()))?,
                formatted_issue_date_time: self.formatted_issue_date_time,
            })
        }
    }
    #[derive(Debug)]
    pub struct SupplyChainEventTypeDeserializer {
        occurrence_date_time: Option<super::DateTimeType>,
        state: Box<SupplyChainEventTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum SupplyChainEventTypeDeserializerState {
        Init__,
        OccurrenceDateTime(Option<<super::DateTimeType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl SupplyChainEventTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
            }
            Ok(Self {
                occurrence_date_time: None,
                state: Box::new(SupplyChainEventTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: SupplyChainEventTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use SupplyChainEventTypeDeserializerState as S;
            match state {
                S::OccurrenceDateTime(Some(deserializer)) => {
                    self.store_occurrence_date_time(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_occurrence_date_time(&mut self, value: super::DateTimeType) -> Result<(), Error> {
            if self.occurrence_date_time.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"OccurrenceDateTime",
                )))?;
            }
            self.occurrence_date_time = Some(value);
            Ok(())
        }
        fn handle_occurrence_date_time<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::DateTimeType>,
            fallback: &mut Option<SupplyChainEventTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.occurrence_date_time.is_some() {
                    fallback.get_or_insert(
                        SupplyChainEventTypeDeserializerState::OccurrenceDateTime(None),
                    );
                    *self.state = SupplyChainEventTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = SupplyChainEventTypeDeserializerState::OccurrenceDateTime(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_occurrence_date_time(data)?;
                    *self.state = SupplyChainEventTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                SupplyChainEventTypeDeserializerState::OccurrenceDateTime(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = SupplyChainEventTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = SupplyChainEventTypeDeserializerState::OccurrenceDateTime(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::SupplyChainEventType> for SupplyChainEventTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SupplyChainEventType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SupplyChainEventType>
        where
            R: DeserializeReader,
        {
            use SupplyChainEventTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::OccurrenceDateTime(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_occurrence_date_time(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state =
                            SupplyChainEventTypeDeserializerState::OccurrenceDateTime(None);
                        event
                    }
                    (S::OccurrenceDateTime(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"OccurrenceDateTime",
                        ) {
                            let output =
                                <super::DateTimeType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_occurrence_date_time(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::SupplyChainEventType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                SupplyChainEventTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::SupplyChainEventType {
                occurrence_date_time: self
                    .occurrence_date_time
                    .ok_or_else(|| ErrorKind::MissingElement("OccurrenceDateTime".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub struct CurrencyCodeTypeDeserializer {
        content: Option<String>,
        state: Box<CurrencyCodeTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum CurrencyCodeTypeDeserializerState {
        Init__,
        Content__(<String as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl CurrencyCodeTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
            }
            Ok(Self {
                content: None,
                state: Box::new(CurrencyCodeTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: CurrencyCodeTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let CurrencyCodeTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: String) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
        ) -> DeserializerResult<'de, super::CurrencyCodeType>
        where
            R: DeserializeReader,
        {
            use CurrencyCodeTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::CurrencyCodeType> for CurrencyCodeTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::CurrencyCodeType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::CurrencyCodeType>
        where
            R: DeserializeReader,
        {
            use CurrencyCodeTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::CurrencyCodeType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                CurrencyCodeTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::CurrencyCodeType {
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct TradeSettlementPaymentMeansTypeDeserializer {
        type_code: Option<super::PaymentMeansCodeType>,
        payer_party_debtor_financial_account: Option<super::DebtorFinancialAccountType>,
        payee_party_creditor_financial_account: Option<super::CreditorFinancialAccountType>,
        state: Box<TradeSettlementPaymentMeansTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum TradeSettlementPaymentMeansTypeDeserializerState {
        Init__,
        TypeCode(Option<<super::PaymentMeansCodeType as WithDeserializer>::Deserializer>),
        PayerPartyDebtorFinancialAccount(
            Option<<super::DebtorFinancialAccountType as WithDeserializer>::Deserializer>,
        ),
        PayeePartyCreditorFinancialAccount(
            Option<<super::CreditorFinancialAccountType as WithDeserializer>::Deserializer>,
        ),
        Done__,
        Unknown__,
    }
    impl TradeSettlementPaymentMeansTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
            }
            Ok(Self {
                type_code: None,
                payer_party_debtor_financial_account: None,
                payee_party_creditor_financial_account: None,
                state: Box::new(TradeSettlementPaymentMeansTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: TradeSettlementPaymentMeansTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use TradeSettlementPaymentMeansTypeDeserializerState as S;
            match state {
                S::TypeCode(Some(deserializer)) => {
                    self.store_type_code(deserializer.finish(reader)?)?
                }
                S::PayerPartyDebtorFinancialAccount(Some(deserializer)) => {
                    self.store_payer_party_debtor_financial_account(deserializer.finish(reader)?)?
                }
                S::PayeePartyCreditorFinancialAccount(Some(deserializer)) => {
                    self.store_payee_party_creditor_financial_account(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_type_code(&mut self, value: super::PaymentMeansCodeType) -> Result<(), Error> {
            if self.type_code.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"TypeCode",
                )))?;
            }
            self.type_code = Some(value);
            Ok(())
        }
        fn store_payer_party_debtor_financial_account(
            &mut self,
            value: super::DebtorFinancialAccountType,
        ) -> Result<(), Error> {
            if self.payer_party_debtor_financial_account.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"PayerPartyDebtorFinancialAccount",
                )))?;
            }
            self.payer_party_debtor_financial_account = Some(value);
            Ok(())
        }
        fn store_payee_party_creditor_financial_account(
            &mut self,
            value: super::CreditorFinancialAccountType,
        ) -> Result<(), Error> {
            if self.payee_party_creditor_financial_account.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"PayeePartyCreditorFinancialAccount",
                )))?;
            }
            self.payee_party_creditor_financial_account = Some(value);
            Ok(())
        }
        fn handle_type_code<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::PaymentMeansCodeType>,
            fallback: &mut Option<TradeSettlementPaymentMeansTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.type_code.is_some() {
                    fallback.get_or_insert(
                        TradeSettlementPaymentMeansTypeDeserializerState::TypeCode(None),
                    );
                    * self . state = TradeSettlementPaymentMeansTypeDeserializerState :: PayerPartyDebtorFinancialAccount (None) ;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = TradeSettlementPaymentMeansTypeDeserializerState::TypeCode(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_type_code(data)?;
                    * self . state = TradeSettlementPaymentMeansTypeDeserializerState :: PayerPartyDebtorFinancialAccount (None) ;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                TradeSettlementPaymentMeansTypeDeserializerState::TypeCode(Some(
                                    deserializer,
                                )),
                            );
                            * self . state = TradeSettlementPaymentMeansTypeDeserializerState :: PayerPartyDebtorFinancialAccount (None) ;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                TradeSettlementPaymentMeansTypeDeserializerState::TypeCode(Some(
                                    deserializer,
                                ));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_payer_party_debtor_financial_account<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::DebtorFinancialAccountType>,
            fallback: &mut Option<TradeSettlementPaymentMeansTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback . get_or_insert (TradeSettlementPaymentMeansTypeDeserializerState :: PayerPartyDebtorFinancialAccount (None)) ;
                * self . state = TradeSettlementPaymentMeansTypeDeserializerState :: PayeePartyCreditorFinancialAccount (None) ;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_payer_party_debtor_financial_account(data)?;
                    * self . state = TradeSettlementPaymentMeansTypeDeserializerState :: PayeePartyCreditorFinancialAccount (None) ;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback . get_or_insert (TradeSettlementPaymentMeansTypeDeserializerState :: PayerPartyDebtorFinancialAccount (Some (deserializer))) ;
                            * self . state = TradeSettlementPaymentMeansTypeDeserializerState :: PayeePartyCreditorFinancialAccount (None) ;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            * self . state = TradeSettlementPaymentMeansTypeDeserializerState :: PayerPartyDebtorFinancialAccount (Some (deserializer)) ;
                        }
                    }
                    ret
                }
            })
        }
        fn handle_payee_party_creditor_financial_account<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::CreditorFinancialAccountType>,
            fallback: &mut Option<TradeSettlementPaymentMeansTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback . get_or_insert (TradeSettlementPaymentMeansTypeDeserializerState :: PayeePartyCreditorFinancialAccount (None)) ;
                *self.state = TradeSettlementPaymentMeansTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_payee_party_creditor_financial_account(data)?;
                    *self.state = TradeSettlementPaymentMeansTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback . get_or_insert (TradeSettlementPaymentMeansTypeDeserializerState :: PayeePartyCreditorFinancialAccount (Some (deserializer))) ;
                            *self.state = TradeSettlementPaymentMeansTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            * self . state = TradeSettlementPaymentMeansTypeDeserializerState :: PayeePartyCreditorFinancialAccount (Some (deserializer)) ;
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::TradeSettlementPaymentMeansType>
        for TradeSettlementPaymentMeansTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TradeSettlementPaymentMeansType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TradeSettlementPaymentMeansType>
        where
            R: DeserializeReader,
        {
            use TradeSettlementPaymentMeansTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::TypeCode(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_type_code(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::PayerPartyDebtorFinancialAccount(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_payer_party_debtor_financial_account(
                            reader,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::PayeePartyCreditorFinancialAccount(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_payee_party_creditor_financial_account(
                            reader,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state =
                            TradeSettlementPaymentMeansTypeDeserializerState::TypeCode(None);
                        event
                    }
                    (S::TypeCode(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_RAM), b"TypeCode") {
                            let output = < super :: PaymentMeansCodeType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_type_code(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::PayerPartyDebtorFinancialAccount(None);
                            event
                        }
                    }
                    (
                        S::PayerPartyDebtorFinancialAccount(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"PayerPartyDebtorFinancialAccount",
                        ) {
                            let output = < super :: DebtorFinancialAccountType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_payer_party_debtor_financial_account(
                                reader,
                                output,
                                &mut fallback,
                            )? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::PayeePartyCreditorFinancialAccount(None);
                            event
                        }
                    }
                    (
                        S::PayeePartyCreditorFinancialAccount(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"PayeePartyCreditorFinancialAccount",
                        ) {
                            let output = < super :: CreditorFinancialAccountType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_payee_party_creditor_financial_account(
                                reader,
                                output,
                                &mut fallback,
                            )? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::TradeSettlementPaymentMeansType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                TradeSettlementPaymentMeansTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::TradeSettlementPaymentMeansType {
                type_code: self
                    .type_code
                    .ok_or_else(|| ErrorKind::MissingElement("TypeCode".into()))?,
                payer_party_debtor_financial_account: self.payer_party_debtor_financial_account,
                payee_party_creditor_financial_account: self.payee_party_creditor_financial_account,
            })
        }
    }
    #[derive(Debug)]
    pub struct TradeTaxTypeDeserializer {
        calculated_amount: Option<super::AmountType>,
        type_code: Option<super::TaxTypeCodeType>,
        exemption_reason: Option<super::TextType>,
        basis_amount: Option<super::AmountType>,
        category_code: Option<super::TaxCategoryCodeType>,
        exemption_reason_code: Option<super::CodeType>,
        due_date_type_code: Option<super::TimeReferenceCodeType>,
        rate_applicable_percent: Option<super::PercentType>,
        state: Box<TradeTaxTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum TradeTaxTypeDeserializerState {
        Init__,
        CalculatedAmount(Option<<super::AmountType as WithDeserializer>::Deserializer>),
        TypeCode(Option<<super::TaxTypeCodeType as WithDeserializer>::Deserializer>),
        ExemptionReason(Option<<super::TextType as WithDeserializer>::Deserializer>),
        BasisAmount(Option<<super::AmountType as WithDeserializer>::Deserializer>),
        CategoryCode(Option<<super::TaxCategoryCodeType as WithDeserializer>::Deserializer>),
        ExemptionReasonCode(Option<<super::CodeType as WithDeserializer>::Deserializer>),
        DueDateTypeCode(Option<<super::TimeReferenceCodeType as WithDeserializer>::Deserializer>),
        RateApplicablePercent(Option<<super::PercentType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl TradeTaxTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
            }
            Ok(Self {
                calculated_amount: None,
                type_code: None,
                exemption_reason: None,
                basis_amount: None,
                category_code: None,
                exemption_reason_code: None,
                due_date_type_code: None,
                rate_applicable_percent: None,
                state: Box::new(TradeTaxTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: TradeTaxTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use TradeTaxTypeDeserializerState as S;
            match state {
                S::CalculatedAmount(Some(deserializer)) => {
                    self.store_calculated_amount(deserializer.finish(reader)?)?
                }
                S::TypeCode(Some(deserializer)) => {
                    self.store_type_code(deserializer.finish(reader)?)?
                }
                S::ExemptionReason(Some(deserializer)) => {
                    self.store_exemption_reason(deserializer.finish(reader)?)?
                }
                S::BasisAmount(Some(deserializer)) => {
                    self.store_basis_amount(deserializer.finish(reader)?)?
                }
                S::CategoryCode(Some(deserializer)) => {
                    self.store_category_code(deserializer.finish(reader)?)?
                }
                S::ExemptionReasonCode(Some(deserializer)) => {
                    self.store_exemption_reason_code(deserializer.finish(reader)?)?
                }
                S::DueDateTypeCode(Some(deserializer)) => {
                    self.store_due_date_type_code(deserializer.finish(reader)?)?
                }
                S::RateApplicablePercent(Some(deserializer)) => {
                    self.store_rate_applicable_percent(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_calculated_amount(&mut self, value: super::AmountType) -> Result<(), Error> {
            if self.calculated_amount.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"CalculatedAmount",
                )))?;
            }
            self.calculated_amount = Some(value);
            Ok(())
        }
        fn store_type_code(&mut self, value: super::TaxTypeCodeType) -> Result<(), Error> {
            if self.type_code.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"TypeCode",
                )))?;
            }
            self.type_code = Some(value);
            Ok(())
        }
        fn store_exemption_reason(&mut self, value: super::TextType) -> Result<(), Error> {
            if self.exemption_reason.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"ExemptionReason",
                )))?;
            }
            self.exemption_reason = Some(value);
            Ok(())
        }
        fn store_basis_amount(&mut self, value: super::AmountType) -> Result<(), Error> {
            if self.basis_amount.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"BasisAmount",
                )))?;
            }
            self.basis_amount = Some(value);
            Ok(())
        }
        fn store_category_code(&mut self, value: super::TaxCategoryCodeType) -> Result<(), Error> {
            if self.category_code.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"CategoryCode",
                )))?;
            }
            self.category_code = Some(value);
            Ok(())
        }
        fn store_exemption_reason_code(&mut self, value: super::CodeType) -> Result<(), Error> {
            if self.exemption_reason_code.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"ExemptionReasonCode",
                )))?;
            }
            self.exemption_reason_code = Some(value);
            Ok(())
        }
        fn store_due_date_type_code(
            &mut self,
            value: super::TimeReferenceCodeType,
        ) -> Result<(), Error> {
            if self.due_date_type_code.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"DueDateTypeCode",
                )))?;
            }
            self.due_date_type_code = Some(value);
            Ok(())
        }
        fn store_rate_applicable_percent(
            &mut self,
            value: super::PercentType,
        ) -> Result<(), Error> {
            if self.rate_applicable_percent.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"RateApplicablePercent",
                )))?;
            }
            self.rate_applicable_percent = Some(value);
            Ok(())
        }
        fn handle_calculated_amount<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::AmountType>,
            fallback: &mut Option<TradeTaxTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(TradeTaxTypeDeserializerState::CalculatedAmount(None));
                *self.state = TradeTaxTypeDeserializerState::TypeCode(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_calculated_amount(data)?;
                    *self.state = TradeTaxTypeDeserializerState::TypeCode(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                TradeTaxTypeDeserializerState::CalculatedAmount(Some(deserializer)),
                            );
                            *self.state = TradeTaxTypeDeserializerState::TypeCode(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                TradeTaxTypeDeserializerState::CalculatedAmount(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_type_code<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::TaxTypeCodeType>,
            fallback: &mut Option<TradeTaxTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.type_code.is_some() {
                    fallback.get_or_insert(TradeTaxTypeDeserializerState::TypeCode(None));
                    *self.state = TradeTaxTypeDeserializerState::ExemptionReason(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = TradeTaxTypeDeserializerState::TypeCode(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_type_code(data)?;
                    *self.state = TradeTaxTypeDeserializerState::ExemptionReason(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(TradeTaxTypeDeserializerState::TypeCode(Some(
                                deserializer,
                            )));
                            *self.state = TradeTaxTypeDeserializerState::ExemptionReason(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                TradeTaxTypeDeserializerState::TypeCode(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_exemption_reason<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::TextType>,
            fallback: &mut Option<TradeTaxTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(TradeTaxTypeDeserializerState::ExemptionReason(None));
                *self.state = TradeTaxTypeDeserializerState::BasisAmount(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_exemption_reason(data)?;
                    *self.state = TradeTaxTypeDeserializerState::BasisAmount(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(TradeTaxTypeDeserializerState::ExemptionReason(
                                Some(deserializer),
                            ));
                            *self.state = TradeTaxTypeDeserializerState::BasisAmount(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                TradeTaxTypeDeserializerState::ExemptionReason(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_basis_amount<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::AmountType>,
            fallback: &mut Option<TradeTaxTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(TradeTaxTypeDeserializerState::BasisAmount(None));
                *self.state = TradeTaxTypeDeserializerState::CategoryCode(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_basis_amount(data)?;
                    *self.state = TradeTaxTypeDeserializerState::CategoryCode(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(TradeTaxTypeDeserializerState::BasisAmount(
                                Some(deserializer),
                            ));
                            *self.state = TradeTaxTypeDeserializerState::CategoryCode(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                TradeTaxTypeDeserializerState::BasisAmount(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_category_code<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::TaxCategoryCodeType>,
            fallback: &mut Option<TradeTaxTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.category_code.is_some() {
                    fallback.get_or_insert(TradeTaxTypeDeserializerState::CategoryCode(None));
                    *self.state = TradeTaxTypeDeserializerState::ExemptionReasonCode(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = TradeTaxTypeDeserializerState::CategoryCode(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_category_code(data)?;
                    *self.state = TradeTaxTypeDeserializerState::ExemptionReasonCode(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(TradeTaxTypeDeserializerState::CategoryCode(
                                Some(deserializer),
                            ));
                            *self.state = TradeTaxTypeDeserializerState::ExemptionReasonCode(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                TradeTaxTypeDeserializerState::CategoryCode(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_exemption_reason_code<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::CodeType>,
            fallback: &mut Option<TradeTaxTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(TradeTaxTypeDeserializerState::ExemptionReasonCode(None));
                *self.state = TradeTaxTypeDeserializerState::DueDateTypeCode(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_exemption_reason_code(data)?;
                    *self.state = TradeTaxTypeDeserializerState::DueDateTypeCode(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                TradeTaxTypeDeserializerState::ExemptionReasonCode(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = TradeTaxTypeDeserializerState::DueDateTypeCode(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = TradeTaxTypeDeserializerState::ExemptionReasonCode(Some(
                                deserializer,
                            ));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_due_date_type_code<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::TimeReferenceCodeType>,
            fallback: &mut Option<TradeTaxTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(TradeTaxTypeDeserializerState::DueDateTypeCode(None));
                *self.state = TradeTaxTypeDeserializerState::RateApplicablePercent(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_due_date_type_code(data)?;
                    *self.state = TradeTaxTypeDeserializerState::RateApplicablePercent(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(TradeTaxTypeDeserializerState::DueDateTypeCode(
                                Some(deserializer),
                            ));
                            *self.state =
                                TradeTaxTypeDeserializerState::RateApplicablePercent(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                TradeTaxTypeDeserializerState::DueDateTypeCode(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_rate_applicable_percent<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::PercentType>,
            fallback: &mut Option<TradeTaxTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(TradeTaxTypeDeserializerState::RateApplicablePercent(None));
                *self.state = TradeTaxTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_rate_applicable_percent(data)?;
                    *self.state = TradeTaxTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                TradeTaxTypeDeserializerState::RateApplicablePercent(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = TradeTaxTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = TradeTaxTypeDeserializerState::RateApplicablePercent(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::TradeTaxType> for TradeTaxTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::TradeTaxType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TradeTaxType>
        where
            R: DeserializeReader,
        {
            use TradeTaxTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::CalculatedAmount(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_calculated_amount(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::TypeCode(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_type_code(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::ExemptionReason(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_exemption_reason(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::BasisAmount(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_basis_amount(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::CategoryCode(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_category_code(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::ExemptionReasonCode(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_exemption_reason_code(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::DueDateTypeCode(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_due_date_type_code(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::RateApplicablePercent(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_rate_applicable_percent(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = TradeTaxTypeDeserializerState::CalculatedAmount(None);
                        event
                    }
                    (S::CalculatedAmount(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"CalculatedAmount",
                        ) {
                            let output =
                                <super::AmountType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_calculated_amount(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::TypeCode(None);
                            event
                        }
                    }
                    (S::TypeCode(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_RAM), b"TypeCode") {
                            let output =
                                <super::TaxTypeCodeType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_type_code(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::ExemptionReason(None);
                            event
                        }
                    }
                    (S::ExemptionReason(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"ExemptionReason",
                        ) {
                            let output = <super::TextType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            match self.handle_exemption_reason(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::BasisAmount(None);
                            event
                        }
                    }
                    (S::BasisAmount(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_RAM), b"BasisAmount")
                        {
                            let output =
                                <super::AmountType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_basis_amount(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::CategoryCode(None);
                            event
                        }
                    }
                    (S::CategoryCode(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"CategoryCode",
                        ) {
                            let output = < super :: TaxCategoryCodeType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_category_code(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::ExemptionReasonCode(None);
                            event
                        }
                    }
                    (S::ExemptionReasonCode(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"ExemptionReasonCode",
                        ) {
                            let output = <super::CodeType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            match self.handle_exemption_reason_code(
                                reader,
                                output,
                                &mut fallback,
                            )? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::DueDateTypeCode(None);
                            event
                        }
                    }
                    (S::DueDateTypeCode(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"DueDateTypeCode",
                        ) {
                            let output = < super :: TimeReferenceCodeType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_due_date_type_code(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::RateApplicablePercent(None);
                            event
                        }
                    }
                    (
                        S::RateApplicablePercent(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"RateApplicablePercent",
                        ) {
                            let output =
                                <super::PercentType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_rate_applicable_percent(
                                reader,
                                output,
                                &mut fallback,
                            )? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::TradeTaxType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, TradeTaxTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::TradeTaxType {
                calculated_amount: self.calculated_amount,
                type_code: self
                    .type_code
                    .ok_or_else(|| ErrorKind::MissingElement("TypeCode".into()))?,
                exemption_reason: self.exemption_reason,
                basis_amount: self.basis_amount,
                category_code: self
                    .category_code
                    .ok_or_else(|| ErrorKind::MissingElement("CategoryCode".into()))?,
                exemption_reason_code: self.exemption_reason_code,
                due_date_type_code: self.due_date_type_code,
                rate_applicable_percent: self.rate_applicable_percent,
            })
        }
    }
    #[derive(Debug)]
    pub struct SpecifiedPeriodTypeDeserializer {
        start_date_time: Option<super::DateTimeType>,
        end_date_time: Option<super::DateTimeType>,
        state: Box<SpecifiedPeriodTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum SpecifiedPeriodTypeDeserializerState {
        Init__,
        StartDateTime(Option<<super::DateTimeType as WithDeserializer>::Deserializer>),
        EndDateTime(Option<<super::DateTimeType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl SpecifiedPeriodTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
            }
            Ok(Self {
                start_date_time: None,
                end_date_time: None,
                state: Box::new(SpecifiedPeriodTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: SpecifiedPeriodTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use SpecifiedPeriodTypeDeserializerState as S;
            match state {
                S::StartDateTime(Some(deserializer)) => {
                    self.store_start_date_time(deserializer.finish(reader)?)?
                }
                S::EndDateTime(Some(deserializer)) => {
                    self.store_end_date_time(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_start_date_time(&mut self, value: super::DateTimeType) -> Result<(), Error> {
            if self.start_date_time.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"StartDateTime",
                )))?;
            }
            self.start_date_time = Some(value);
            Ok(())
        }
        fn store_end_date_time(&mut self, value: super::DateTimeType) -> Result<(), Error> {
            if self.end_date_time.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"EndDateTime",
                )))?;
            }
            self.end_date_time = Some(value);
            Ok(())
        }
        fn handle_start_date_time<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::DateTimeType>,
            fallback: &mut Option<SpecifiedPeriodTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(SpecifiedPeriodTypeDeserializerState::StartDateTime(None));
                *self.state = SpecifiedPeriodTypeDeserializerState::EndDateTime(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_start_date_time(data)?;
                    *self.state = SpecifiedPeriodTypeDeserializerState::EndDateTime(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                SpecifiedPeriodTypeDeserializerState::StartDateTime(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = SpecifiedPeriodTypeDeserializerState::EndDateTime(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = SpecifiedPeriodTypeDeserializerState::StartDateTime(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
        fn handle_end_date_time<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::DateTimeType>,
            fallback: &mut Option<SpecifiedPeriodTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(SpecifiedPeriodTypeDeserializerState::EndDateTime(None));
                *self.state = SpecifiedPeriodTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_end_date_time(data)?;
                    *self.state = SpecifiedPeriodTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                SpecifiedPeriodTypeDeserializerState::EndDateTime(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = SpecifiedPeriodTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = SpecifiedPeriodTypeDeserializerState::EndDateTime(Some(
                                deserializer,
                            ));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::SpecifiedPeriodType> for SpecifiedPeriodTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SpecifiedPeriodType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::SpecifiedPeriodType>
        where
            R: DeserializeReader,
        {
            use SpecifiedPeriodTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::StartDateTime(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_start_date_time(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::EndDateTime(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_end_date_time(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = SpecifiedPeriodTypeDeserializerState::StartDateTime(None);
                        event
                    }
                    (S::StartDateTime(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"StartDateTime",
                        ) {
                            let output =
                                <super::DateTimeType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_start_date_time(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::EndDateTime(None);
                            event
                        }
                    }
                    (S::EndDateTime(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_RAM), b"EndDateTime")
                        {
                            let output =
                                <super::DateTimeType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_end_date_time(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::SpecifiedPeriodType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                SpecifiedPeriodTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::SpecifiedPeriodType {
                start_date_time: self.start_date_time,
                end_date_time: self.end_date_time,
            })
        }
    }
    #[derive(Debug)]
    pub struct TradeAllowanceChargeTypeDeserializer {
        charge_indicator: Option<super::IndicatorType>,
        calculation_percent: Option<super::PercentType>,
        basis_amount: Option<super::AmountType>,
        actual_amount: Option<super::AmountType>,
        reason_code: Option<super::AllowanceChargeReasonCodeType>,
        reason: Option<super::TextType>,
        category_trade_tax: Option<super::TradeTaxType>,
        state: Box<TradeAllowanceChargeTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum TradeAllowanceChargeTypeDeserializerState {
        Init__,
        ChargeIndicator(Option<<super::IndicatorType as WithDeserializer>::Deserializer>),
        CalculationPercent(Option<<super::PercentType as WithDeserializer>::Deserializer>),
        BasisAmount(Option<<super::AmountType as WithDeserializer>::Deserializer>),
        ActualAmount(Option<<super::AmountType as WithDeserializer>::Deserializer>),
        ReasonCode(
            Option<<super::AllowanceChargeReasonCodeType as WithDeserializer>::Deserializer>,
        ),
        Reason(Option<<super::TextType as WithDeserializer>::Deserializer>),
        CategoryTradeTax(Option<<super::TradeTaxType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl TradeAllowanceChargeTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
            }
            Ok(Self {
                charge_indicator: None,
                calculation_percent: None,
                basis_amount: None,
                actual_amount: None,
                reason_code: None,
                reason: None,
                category_trade_tax: None,
                state: Box::new(TradeAllowanceChargeTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: TradeAllowanceChargeTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use TradeAllowanceChargeTypeDeserializerState as S;
            match state {
                S::ChargeIndicator(Some(deserializer)) => {
                    self.store_charge_indicator(deserializer.finish(reader)?)?
                }
                S::CalculationPercent(Some(deserializer)) => {
                    self.store_calculation_percent(deserializer.finish(reader)?)?
                }
                S::BasisAmount(Some(deserializer)) => {
                    self.store_basis_amount(deserializer.finish(reader)?)?
                }
                S::ActualAmount(Some(deserializer)) => {
                    self.store_actual_amount(deserializer.finish(reader)?)?
                }
                S::ReasonCode(Some(deserializer)) => {
                    self.store_reason_code(deserializer.finish(reader)?)?
                }
                S::Reason(Some(deserializer)) => self.store_reason(deserializer.finish(reader)?)?,
                S::CategoryTradeTax(Some(deserializer)) => {
                    self.store_category_trade_tax(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_charge_indicator(&mut self, value: super::IndicatorType) -> Result<(), Error> {
            if self.charge_indicator.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"ChargeIndicator",
                )))?;
            }
            self.charge_indicator = Some(value);
            Ok(())
        }
        fn store_calculation_percent(&mut self, value: super::PercentType) -> Result<(), Error> {
            if self.calculation_percent.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"CalculationPercent",
                )))?;
            }
            self.calculation_percent = Some(value);
            Ok(())
        }
        fn store_basis_amount(&mut self, value: super::AmountType) -> Result<(), Error> {
            if self.basis_amount.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"BasisAmount",
                )))?;
            }
            self.basis_amount = Some(value);
            Ok(())
        }
        fn store_actual_amount(&mut self, value: super::AmountType) -> Result<(), Error> {
            if self.actual_amount.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"ActualAmount",
                )))?;
            }
            self.actual_amount = Some(value);
            Ok(())
        }
        fn store_reason_code(
            &mut self,
            value: super::AllowanceChargeReasonCodeType,
        ) -> Result<(), Error> {
            if self.reason_code.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"ReasonCode",
                )))?;
            }
            self.reason_code = Some(value);
            Ok(())
        }
        fn store_reason(&mut self, value: super::TextType) -> Result<(), Error> {
            if self.reason.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Reason",
                )))?;
            }
            self.reason = Some(value);
            Ok(())
        }
        fn store_category_trade_tax(&mut self, value: super::TradeTaxType) -> Result<(), Error> {
            if self.category_trade_tax.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"CategoryTradeTax",
                )))?;
            }
            self.category_trade_tax = Some(value);
            Ok(())
        }
        fn handle_charge_indicator<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::IndicatorType>,
            fallback: &mut Option<TradeAllowanceChargeTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.charge_indicator.is_some() {
                    fallback.get_or_insert(
                        TradeAllowanceChargeTypeDeserializerState::ChargeIndicator(None),
                    );
                    *self.state =
                        TradeAllowanceChargeTypeDeserializerState::CalculationPercent(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = TradeAllowanceChargeTypeDeserializerState::ChargeIndicator(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_charge_indicator(data)?;
                    *self.state =
                        TradeAllowanceChargeTypeDeserializerState::CalculationPercent(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                TradeAllowanceChargeTypeDeserializerState::ChargeIndicator(Some(
                                    deserializer,
                                )),
                            );
                            *self.state =
                                TradeAllowanceChargeTypeDeserializerState::CalculationPercent(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                TradeAllowanceChargeTypeDeserializerState::ChargeIndicator(Some(
                                    deserializer,
                                ));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_calculation_percent<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::PercentType>,
            fallback: &mut Option<TradeAllowanceChargeTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(
                    TradeAllowanceChargeTypeDeserializerState::CalculationPercent(None),
                );
                *self.state = TradeAllowanceChargeTypeDeserializerState::BasisAmount(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_calculation_percent(data)?;
                    *self.state = TradeAllowanceChargeTypeDeserializerState::BasisAmount(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                TradeAllowanceChargeTypeDeserializerState::CalculationPercent(
                                    Some(deserializer),
                                ),
                            );
                            *self.state =
                                TradeAllowanceChargeTypeDeserializerState::BasisAmount(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                TradeAllowanceChargeTypeDeserializerState::CalculationPercent(
                                    Some(deserializer),
                                );
                        }
                    }
                    ret
                }
            })
        }
        fn handle_basis_amount<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::AmountType>,
            fallback: &mut Option<TradeAllowanceChargeTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback
                    .get_or_insert(TradeAllowanceChargeTypeDeserializerState::BasisAmount(None));
                *self.state = TradeAllowanceChargeTypeDeserializerState::ActualAmount(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_basis_amount(data)?;
                    *self.state = TradeAllowanceChargeTypeDeserializerState::ActualAmount(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                TradeAllowanceChargeTypeDeserializerState::BasisAmount(Some(
                                    deserializer,
                                )),
                            );
                            *self.state =
                                TradeAllowanceChargeTypeDeserializerState::ActualAmount(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = TradeAllowanceChargeTypeDeserializerState::BasisAmount(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
        fn handle_actual_amount<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::AmountType>,
            fallback: &mut Option<TradeAllowanceChargeTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.actual_amount.is_some() {
                    fallback.get_or_insert(
                        TradeAllowanceChargeTypeDeserializerState::ActualAmount(None),
                    );
                    *self.state = TradeAllowanceChargeTypeDeserializerState::ReasonCode(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = TradeAllowanceChargeTypeDeserializerState::ActualAmount(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_actual_amount(data)?;
                    *self.state = TradeAllowanceChargeTypeDeserializerState::ReasonCode(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                TradeAllowanceChargeTypeDeserializerState::ActualAmount(Some(
                                    deserializer,
                                )),
                            );
                            *self.state =
                                TradeAllowanceChargeTypeDeserializerState::ReasonCode(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = TradeAllowanceChargeTypeDeserializerState::ActualAmount(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
        fn handle_reason_code<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::AllowanceChargeReasonCodeType>,
            fallback: &mut Option<TradeAllowanceChargeTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(TradeAllowanceChargeTypeDeserializerState::ReasonCode(None));
                *self.state = TradeAllowanceChargeTypeDeserializerState::Reason(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_reason_code(data)?;
                    *self.state = TradeAllowanceChargeTypeDeserializerState::Reason(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                TradeAllowanceChargeTypeDeserializerState::ReasonCode(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = TradeAllowanceChargeTypeDeserializerState::Reason(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = TradeAllowanceChargeTypeDeserializerState::ReasonCode(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
        fn handle_reason<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::TextType>,
            fallback: &mut Option<TradeAllowanceChargeTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(TradeAllowanceChargeTypeDeserializerState::Reason(None));
                *self.state = TradeAllowanceChargeTypeDeserializerState::CategoryTradeTax(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_reason(data)?;
                    *self.state = TradeAllowanceChargeTypeDeserializerState::CategoryTradeTax(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                TradeAllowanceChargeTypeDeserializerState::Reason(Some(
                                    deserializer,
                                )),
                            );
                            *self.state =
                                TradeAllowanceChargeTypeDeserializerState::CategoryTradeTax(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = TradeAllowanceChargeTypeDeserializerState::Reason(Some(
                                deserializer,
                            ));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_category_trade_tax<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::TradeTaxType>,
            fallback: &mut Option<TradeAllowanceChargeTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.category_trade_tax.is_some() {
                    fallback.get_or_insert(
                        TradeAllowanceChargeTypeDeserializerState::CategoryTradeTax(None),
                    );
                    *self.state = TradeAllowanceChargeTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = TradeAllowanceChargeTypeDeserializerState::CategoryTradeTax(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_category_trade_tax(data)?;
                    *self.state = TradeAllowanceChargeTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                TradeAllowanceChargeTypeDeserializerState::CategoryTradeTax(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = TradeAllowanceChargeTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                TradeAllowanceChargeTypeDeserializerState::CategoryTradeTax(Some(
                                    deserializer,
                                ));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::TradeAllowanceChargeType>
        for TradeAllowanceChargeTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TradeAllowanceChargeType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TradeAllowanceChargeType>
        where
            R: DeserializeReader,
        {
            use TradeAllowanceChargeTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::ChargeIndicator(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_charge_indicator(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::CalculationPercent(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_calculation_percent(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::BasisAmount(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_basis_amount(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::ActualAmount(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_actual_amount(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::ReasonCode(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_reason_code(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Reason(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_reason(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::CategoryTradeTax(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_category_trade_tax(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state =
                            TradeAllowanceChargeTypeDeserializerState::ChargeIndicator(None);
                        event
                    }
                    (S::ChargeIndicator(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"ChargeIndicator",
                        ) {
                            let output =
                                <super::IndicatorType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_charge_indicator(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::CalculationPercent(None);
                            event
                        }
                    }
                    (S::CalculationPercent(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"CalculationPercent",
                        ) {
                            let output =
                                <super::PercentType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_calculation_percent(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::BasisAmount(None);
                            event
                        }
                    }
                    (S::BasisAmount(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_RAM), b"BasisAmount")
                        {
                            let output =
                                <super::AmountType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_basis_amount(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::ActualAmount(None);
                            event
                        }
                    }
                    (S::ActualAmount(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"ActualAmount",
                        ) {
                            let output =
                                <super::AmountType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_actual_amount(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::ReasonCode(None);
                            event
                        }
                    }
                    (S::ReasonCode(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_RAM), b"ReasonCode")
                        {
                            let output = < super :: AllowanceChargeReasonCodeType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_reason_code(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Reason(None);
                            event
                        }
                    }
                    (S::Reason(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_RAM), b"Reason") {
                            let output = <super::TextType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            match self.handle_reason(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::CategoryTradeTax(None);
                            event
                        }
                    }
                    (S::CategoryTradeTax(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"CategoryTradeTax",
                        ) {
                            let output =
                                <super::TradeTaxType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_category_trade_tax(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::TradeAllowanceChargeType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                TradeAllowanceChargeTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::TradeAllowanceChargeType {
                charge_indicator: self
                    .charge_indicator
                    .ok_or_else(|| ErrorKind::MissingElement("ChargeIndicator".into()))?,
                calculation_percent: self.calculation_percent,
                basis_amount: self.basis_amount,
                actual_amount: self
                    .actual_amount
                    .ok_or_else(|| ErrorKind::MissingElement("ActualAmount".into()))?,
                reason_code: self.reason_code,
                reason: self.reason,
                category_trade_tax: self
                    .category_trade_tax
                    .ok_or_else(|| ErrorKind::MissingElement("CategoryTradeTax".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub struct TradePaymentTermsTypeDeserializer {
        description: Option<super::TextType>,
        due_date_date_time: Option<super::DateTimeType>,
        direct_debit_mandate_id: Option<super::IdType>,
        state: Box<TradePaymentTermsTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum TradePaymentTermsTypeDeserializerState {
        Init__,
        Description(Option<<super::TextType as WithDeserializer>::Deserializer>),
        DueDateDateTime(Option<<super::DateTimeType as WithDeserializer>::Deserializer>),
        DirectDebitMandateId(Option<<super::IdType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl TradePaymentTermsTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
            }
            Ok(Self {
                description: None,
                due_date_date_time: None,
                direct_debit_mandate_id: None,
                state: Box::new(TradePaymentTermsTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: TradePaymentTermsTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use TradePaymentTermsTypeDeserializerState as S;
            match state {
                S::Description(Some(deserializer)) => {
                    self.store_description(deserializer.finish(reader)?)?
                }
                S::DueDateDateTime(Some(deserializer)) => {
                    self.store_due_date_date_time(deserializer.finish(reader)?)?
                }
                S::DirectDebitMandateId(Some(deserializer)) => {
                    self.store_direct_debit_mandate_id(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_description(&mut self, value: super::TextType) -> Result<(), Error> {
            if self.description.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Description",
                )))?;
            }
            self.description = Some(value);
            Ok(())
        }
        fn store_due_date_date_time(&mut self, value: super::DateTimeType) -> Result<(), Error> {
            if self.due_date_date_time.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"DueDateDateTime",
                )))?;
            }
            self.due_date_date_time = Some(value);
            Ok(())
        }
        fn store_direct_debit_mandate_id(&mut self, value: super::IdType) -> Result<(), Error> {
            if self.direct_debit_mandate_id.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"DirectDebitMandateID",
                )))?;
            }
            self.direct_debit_mandate_id = Some(value);
            Ok(())
        }
        fn handle_description<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::TextType>,
            fallback: &mut Option<TradePaymentTermsTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(TradePaymentTermsTypeDeserializerState::Description(None));
                *self.state = TradePaymentTermsTypeDeserializerState::DueDateDateTime(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_description(data)?;
                    *self.state = TradePaymentTermsTypeDeserializerState::DueDateDateTime(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                TradePaymentTermsTypeDeserializerState::Description(Some(
                                    deserializer,
                                )),
                            );
                            *self.state =
                                TradePaymentTermsTypeDeserializerState::DueDateDateTime(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = TradePaymentTermsTypeDeserializerState::Description(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
        fn handle_due_date_date_time<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::DateTimeType>,
            fallback: &mut Option<TradePaymentTermsTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(TradePaymentTermsTypeDeserializerState::DueDateDateTime(
                    None,
                ));
                *self.state = TradePaymentTermsTypeDeserializerState::DirectDebitMandateId(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_due_date_date_time(data)?;
                    *self.state =
                        TradePaymentTermsTypeDeserializerState::DirectDebitMandateId(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                TradePaymentTermsTypeDeserializerState::DueDateDateTime(Some(
                                    deserializer,
                                )),
                            );
                            *self.state =
                                TradePaymentTermsTypeDeserializerState::DirectDebitMandateId(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = TradePaymentTermsTypeDeserializerState::DueDateDateTime(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
        fn handle_direct_debit_mandate_id<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::IdType>,
            fallback: &mut Option<TradePaymentTermsTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(
                    TradePaymentTermsTypeDeserializerState::DirectDebitMandateId(None),
                );
                *self.state = TradePaymentTermsTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_direct_debit_mandate_id(data)?;
                    *self.state = TradePaymentTermsTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                TradePaymentTermsTypeDeserializerState::DirectDebitMandateId(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = TradePaymentTermsTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                TradePaymentTermsTypeDeserializerState::DirectDebitMandateId(Some(
                                    deserializer,
                                ));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::TradePaymentTermsType> for TradePaymentTermsTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TradePaymentTermsType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TradePaymentTermsType>
        where
            R: DeserializeReader,
        {
            use TradePaymentTermsTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Description(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_description(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::DueDateDateTime(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_due_date_date_time(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::DirectDebitMandateId(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_direct_debit_mandate_id(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = TradePaymentTermsTypeDeserializerState::Description(None);
                        event
                    }
                    (S::Description(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_RAM), b"Description")
                        {
                            let output = <super::TextType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            match self.handle_description(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::DueDateDateTime(None);
                            event
                        }
                    }
                    (S::DueDateDateTime(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"DueDateDateTime",
                        ) {
                            let output =
                                <super::DateTimeType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_due_date_date_time(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::DirectDebitMandateId(None);
                            event
                        }
                    }
                    (
                        S::DirectDebitMandateId(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"DirectDebitMandateID",
                        ) {
                            let output = <super::IdType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            match self.handle_direct_debit_mandate_id(
                                reader,
                                output,
                                &mut fallback,
                            )? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::TradePaymentTermsType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                TradePaymentTermsTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::TradePaymentTermsType {
                description: self.description,
                due_date_date_time: self.due_date_date_time,
                direct_debit_mandate_id: self.direct_debit_mandate_id,
            })
        }
    }
    #[derive(Debug)]
    pub struct TradeSettlementHeaderMonetarySummationTypeDeserializer {
        line_total_amount: Option<super::AmountType>,
        charge_total_amount: Option<super::AmountType>,
        allowance_total_amount: Option<super::AmountType>,
        tax_basis_total_amount: Option<super::AmountType>,
        tax_total_amount: Vec<super::AmountType>,
        grand_total_amount: Option<super::AmountType>,
        total_prepaid_amount: Option<super::AmountType>,
        due_payable_amount: Option<super::AmountType>,
        state: Box<TradeSettlementHeaderMonetarySummationTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum TradeSettlementHeaderMonetarySummationTypeDeserializerState {
        Init__,
        LineTotalAmount(Option<<super::AmountType as WithDeserializer>::Deserializer>),
        ChargeTotalAmount(Option<<super::AmountType as WithDeserializer>::Deserializer>),
        AllowanceTotalAmount(Option<<super::AmountType as WithDeserializer>::Deserializer>),
        TaxBasisTotalAmount(Option<<super::AmountType as WithDeserializer>::Deserializer>),
        TaxTotalAmount(Option<<super::AmountType as WithDeserializer>::Deserializer>),
        GrandTotalAmount(Option<<super::AmountType as WithDeserializer>::Deserializer>),
        TotalPrepaidAmount(Option<<super::AmountType as WithDeserializer>::Deserializer>),
        DuePayableAmount(Option<<super::AmountType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl TradeSettlementHeaderMonetarySummationTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
            }
            Ok(Self {
                line_total_amount: None,
                charge_total_amount: None,
                allowance_total_amount: None,
                tax_basis_total_amount: None,
                tax_total_amount: Vec::new(),
                grand_total_amount: None,
                total_prepaid_amount: None,
                due_payable_amount: None,
                state: Box::new(
                    TradeSettlementHeaderMonetarySummationTypeDeserializerState::Init__,
                ),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: TradeSettlementHeaderMonetarySummationTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use TradeSettlementHeaderMonetarySummationTypeDeserializerState as S;
            match state {
                S::LineTotalAmount(Some(deserializer)) => {
                    self.store_line_total_amount(deserializer.finish(reader)?)?
                }
                S::ChargeTotalAmount(Some(deserializer)) => {
                    self.store_charge_total_amount(deserializer.finish(reader)?)?
                }
                S::AllowanceTotalAmount(Some(deserializer)) => {
                    self.store_allowance_total_amount(deserializer.finish(reader)?)?
                }
                S::TaxBasisTotalAmount(Some(deserializer)) => {
                    self.store_tax_basis_total_amount(deserializer.finish(reader)?)?
                }
                S::TaxTotalAmount(Some(deserializer)) => {
                    self.store_tax_total_amount(deserializer.finish(reader)?)?
                }
                S::GrandTotalAmount(Some(deserializer)) => {
                    self.store_grand_total_amount(deserializer.finish(reader)?)?
                }
                S::TotalPrepaidAmount(Some(deserializer)) => {
                    self.store_total_prepaid_amount(deserializer.finish(reader)?)?
                }
                S::DuePayableAmount(Some(deserializer)) => {
                    self.store_due_payable_amount(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_line_total_amount(&mut self, value: super::AmountType) -> Result<(), Error> {
            if self.line_total_amount.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"LineTotalAmount",
                )))?;
            }
            self.line_total_amount = Some(value);
            Ok(())
        }
        fn store_charge_total_amount(&mut self, value: super::AmountType) -> Result<(), Error> {
            if self.charge_total_amount.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"ChargeTotalAmount",
                )))?;
            }
            self.charge_total_amount = Some(value);
            Ok(())
        }
        fn store_allowance_total_amount(&mut self, value: super::AmountType) -> Result<(), Error> {
            if self.allowance_total_amount.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"AllowanceTotalAmount",
                )))?;
            }
            self.allowance_total_amount = Some(value);
            Ok(())
        }
        fn store_tax_basis_total_amount(&mut self, value: super::AmountType) -> Result<(), Error> {
            if self.tax_basis_total_amount.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"TaxBasisTotalAmount",
                )))?;
            }
            self.tax_basis_total_amount = Some(value);
            Ok(())
        }
        fn store_tax_total_amount(&mut self, value: super::AmountType) -> Result<(), Error> {
            self.tax_total_amount.push(value);
            Ok(())
        }
        fn store_grand_total_amount(&mut self, value: super::AmountType) -> Result<(), Error> {
            if self.grand_total_amount.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"GrandTotalAmount",
                )))?;
            }
            self.grand_total_amount = Some(value);
            Ok(())
        }
        fn store_total_prepaid_amount(&mut self, value: super::AmountType) -> Result<(), Error> {
            if self.total_prepaid_amount.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"TotalPrepaidAmount",
                )))?;
            }
            self.total_prepaid_amount = Some(value);
            Ok(())
        }
        fn store_due_payable_amount(&mut self, value: super::AmountType) -> Result<(), Error> {
            if self.due_payable_amount.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"DuePayableAmount",
                )))?;
            }
            self.due_payable_amount = Some(value);
            Ok(())
        }
        fn handle_line_total_amount<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::AmountType>,
            fallback: &mut Option<TradeSettlementHeaderMonetarySummationTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.line_total_amount.is_some() {
                    fallback . get_or_insert (TradeSettlementHeaderMonetarySummationTypeDeserializerState :: LineTotalAmount (None)) ;
                    * self . state = TradeSettlementHeaderMonetarySummationTypeDeserializerState :: ChargeTotalAmount (None) ;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    * self . state = TradeSettlementHeaderMonetarySummationTypeDeserializerState :: LineTotalAmount (None) ;
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_line_total_amount(data)?;
                    * self . state = TradeSettlementHeaderMonetarySummationTypeDeserializerState :: ChargeTotalAmount (None) ;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback . get_or_insert (TradeSettlementHeaderMonetarySummationTypeDeserializerState :: LineTotalAmount (Some (deserializer))) ;
                            * self . state = TradeSettlementHeaderMonetarySummationTypeDeserializerState :: ChargeTotalAmount (None) ;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            * self . state = TradeSettlementHeaderMonetarySummationTypeDeserializerState :: LineTotalAmount (Some (deserializer)) ;
                        }
                    }
                    ret
                }
            })
        }
        fn handle_charge_total_amount<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::AmountType>,
            fallback: &mut Option<TradeSettlementHeaderMonetarySummationTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(
                    TradeSettlementHeaderMonetarySummationTypeDeserializerState::ChargeTotalAmount(
                        None,
                    ),
                );
                * self . state = TradeSettlementHeaderMonetarySummationTypeDeserializerState :: AllowanceTotalAmount (None) ;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_charge_total_amount(data)?;
                    * self . state = TradeSettlementHeaderMonetarySummationTypeDeserializerState :: AllowanceTotalAmount (None) ;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback . get_or_insert (TradeSettlementHeaderMonetarySummationTypeDeserializerState :: ChargeTotalAmount (Some (deserializer))) ;
                            * self . state = TradeSettlementHeaderMonetarySummationTypeDeserializerState :: AllowanceTotalAmount (None) ;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            * self . state = TradeSettlementHeaderMonetarySummationTypeDeserializerState :: ChargeTotalAmount (Some (deserializer)) ;
                        }
                    }
                    ret
                }
            })
        }
        fn handle_allowance_total_amount<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::AmountType>,
            fallback: &mut Option<TradeSettlementHeaderMonetarySummationTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback . get_or_insert (TradeSettlementHeaderMonetarySummationTypeDeserializerState :: AllowanceTotalAmount (None)) ;
                * self . state = TradeSettlementHeaderMonetarySummationTypeDeserializerState :: TaxBasisTotalAmount (None) ;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_allowance_total_amount(data)?;
                    * self . state = TradeSettlementHeaderMonetarySummationTypeDeserializerState :: TaxBasisTotalAmount (None) ;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback . get_or_insert (TradeSettlementHeaderMonetarySummationTypeDeserializerState :: AllowanceTotalAmount (Some (deserializer))) ;
                            * self . state = TradeSettlementHeaderMonetarySummationTypeDeserializerState :: TaxBasisTotalAmount (None) ;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            * self . state = TradeSettlementHeaderMonetarySummationTypeDeserializerState :: AllowanceTotalAmount (Some (deserializer)) ;
                        }
                    }
                    ret
                }
            })
        }
        fn handle_tax_basis_total_amount<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::AmountType>,
            fallback: &mut Option<TradeSettlementHeaderMonetarySummationTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.tax_basis_total_amount.is_some() {
                    fallback . get_or_insert (TradeSettlementHeaderMonetarySummationTypeDeserializerState :: TaxBasisTotalAmount (None)) ;
                    *self.state =
                        TradeSettlementHeaderMonetarySummationTypeDeserializerState::TaxTotalAmount(
                            None,
                        );
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    * self . state = TradeSettlementHeaderMonetarySummationTypeDeserializerState :: TaxBasisTotalAmount (None) ;
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_tax_basis_total_amount(data)?;
                    *self.state =
                        TradeSettlementHeaderMonetarySummationTypeDeserializerState::TaxTotalAmount(
                            None,
                        );
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback . get_or_insert (TradeSettlementHeaderMonetarySummationTypeDeserializerState :: TaxBasisTotalAmount (Some (deserializer))) ;
                            * self . state = TradeSettlementHeaderMonetarySummationTypeDeserializerState :: TaxTotalAmount (None) ;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            * self . state = TradeSettlementHeaderMonetarySummationTypeDeserializerState :: TaxBasisTotalAmount (Some (deserializer)) ;
                        }
                    }
                    ret
                }
            })
        }
        fn handle_tax_total_amount<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::AmountType>,
            fallback: &mut Option<TradeSettlementHeaderMonetarySummationTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(
                    TradeSettlementHeaderMonetarySummationTypeDeserializerState::TaxTotalAmount(
                        None,
                    ),
                );
                *self.state =
                    TradeSettlementHeaderMonetarySummationTypeDeserializerState::GrandTotalAmount(
                        None,
                    );
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_tax_total_amount(data)?;
                    if self.tax_total_amount.len() < 2usize {
                        * self . state = TradeSettlementHeaderMonetarySummationTypeDeserializerState :: TaxTotalAmount (None) ;
                    } else {
                        * self . state = TradeSettlementHeaderMonetarySummationTypeDeserializerState :: GrandTotalAmount (None) ;
                    }
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback . get_or_insert (TradeSettlementHeaderMonetarySummationTypeDeserializerState :: TaxTotalAmount (Some (deserializer))) ;
                            * self . state = TradeSettlementHeaderMonetarySummationTypeDeserializerState :: TaxTotalAmount (None) ;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            * self . state = TradeSettlementHeaderMonetarySummationTypeDeserializerState :: TaxTotalAmount (Some (deserializer)) ;
                        }
                    }
                    ret
                }
            })
        }
        fn handle_grand_total_amount<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::AmountType>,
            fallback: &mut Option<TradeSettlementHeaderMonetarySummationTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.grand_total_amount.is_some() {
                    fallback . get_or_insert (TradeSettlementHeaderMonetarySummationTypeDeserializerState :: GrandTotalAmount (None)) ;
                    * self . state = TradeSettlementHeaderMonetarySummationTypeDeserializerState :: TotalPrepaidAmount (None) ;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    * self . state = TradeSettlementHeaderMonetarySummationTypeDeserializerState :: GrandTotalAmount (None) ;
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_grand_total_amount(data)?;
                    * self . state = TradeSettlementHeaderMonetarySummationTypeDeserializerState :: TotalPrepaidAmount (None) ;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback . get_or_insert (TradeSettlementHeaderMonetarySummationTypeDeserializerState :: GrandTotalAmount (Some (deserializer))) ;
                            * self . state = TradeSettlementHeaderMonetarySummationTypeDeserializerState :: TotalPrepaidAmount (None) ;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            * self . state = TradeSettlementHeaderMonetarySummationTypeDeserializerState :: GrandTotalAmount (Some (deserializer)) ;
                        }
                    }
                    ret
                }
            })
        }
        fn handle_total_prepaid_amount<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::AmountType>,
            fallback: &mut Option<TradeSettlementHeaderMonetarySummationTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(
                    TradeSettlementHeaderMonetarySummationTypeDeserializerState::TotalPrepaidAmount(
                        None,
                    ),
                );
                *self.state =
                    TradeSettlementHeaderMonetarySummationTypeDeserializerState::DuePayableAmount(
                        None,
                    );
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_total_prepaid_amount(data)?;
                    * self . state = TradeSettlementHeaderMonetarySummationTypeDeserializerState :: DuePayableAmount (None) ;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback . get_or_insert (TradeSettlementHeaderMonetarySummationTypeDeserializerState :: TotalPrepaidAmount (Some (deserializer))) ;
                            * self . state = TradeSettlementHeaderMonetarySummationTypeDeserializerState :: DuePayableAmount (None) ;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            * self . state = TradeSettlementHeaderMonetarySummationTypeDeserializerState :: TotalPrepaidAmount (Some (deserializer)) ;
                        }
                    }
                    ret
                }
            })
        }
        fn handle_due_payable_amount<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::AmountType>,
            fallback: &mut Option<TradeSettlementHeaderMonetarySummationTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.due_payable_amount.is_some() {
                    fallback . get_or_insert (TradeSettlementHeaderMonetarySummationTypeDeserializerState :: DuePayableAmount (None)) ;
                    *self.state =
                        TradeSettlementHeaderMonetarySummationTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    * self . state = TradeSettlementHeaderMonetarySummationTypeDeserializerState :: DuePayableAmount (None) ;
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_due_payable_amount(data)?;
                    *self.state =
                        TradeSettlementHeaderMonetarySummationTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback . get_or_insert (TradeSettlementHeaderMonetarySummationTypeDeserializerState :: DuePayableAmount (Some (deserializer))) ;
                            *self.state =
                                TradeSettlementHeaderMonetarySummationTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            * self . state = TradeSettlementHeaderMonetarySummationTypeDeserializerState :: DuePayableAmount (Some (deserializer)) ;
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::TradeSettlementHeaderMonetarySummationType>
        for TradeSettlementHeaderMonetarySummationTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TradeSettlementHeaderMonetarySummationType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TradeSettlementHeaderMonetarySummationType>
        where
            R: DeserializeReader,
        {
            use TradeSettlementHeaderMonetarySummationTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::LineTotalAmount(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_line_total_amount(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::ChargeTotalAmount(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_charge_total_amount(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::AllowanceTotalAmount(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_allowance_total_amount(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::TaxBasisTotalAmount(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_tax_basis_total_amount(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::TaxTotalAmount(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_tax_total_amount(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::GrandTotalAmount(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_grand_total_amount(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::TotalPrepaidAmount(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_total_prepaid_amount(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::DuePayableAmount(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_due_payable_amount(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        * self . state = TradeSettlementHeaderMonetarySummationTypeDeserializerState :: LineTotalAmount (None) ;
                        event
                    }
                    (S::LineTotalAmount(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"LineTotalAmount",
                        ) {
                            let output =
                                <super::AmountType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_line_total_amount(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::ChargeTotalAmount(None);
                            event
                        }
                    }
                    (S::ChargeTotalAmount(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"ChargeTotalAmount",
                        ) {
                            let output =
                                <super::AmountType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_charge_total_amount(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::AllowanceTotalAmount(None);
                            event
                        }
                    }
                    (
                        S::AllowanceTotalAmount(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"AllowanceTotalAmount",
                        ) {
                            let output =
                                <super::AmountType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_allowance_total_amount(
                                reader,
                                output,
                                &mut fallback,
                            )? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::TaxBasisTotalAmount(None);
                            event
                        }
                    }
                    (S::TaxBasisTotalAmount(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"TaxBasisTotalAmount",
                        ) {
                            let output =
                                <super::AmountType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_tax_basis_total_amount(
                                reader,
                                output,
                                &mut fallback,
                            )? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::TaxTotalAmount(None);
                            event
                        }
                    }
                    (S::TaxTotalAmount(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"TaxTotalAmount",
                        ) {
                            let output =
                                <super::AmountType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_tax_total_amount(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::GrandTotalAmount(None);
                            event
                        }
                    }
                    (S::GrandTotalAmount(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"GrandTotalAmount",
                        ) {
                            let output =
                                <super::AmountType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_grand_total_amount(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::TotalPrepaidAmount(None);
                            event
                        }
                    }
                    (S::TotalPrepaidAmount(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"TotalPrepaidAmount",
                        ) {
                            let output =
                                <super::AmountType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_total_prepaid_amount(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::DuePayableAmount(None);
                            event
                        }
                    }
                    (S::DuePayableAmount(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"DuePayableAmount",
                        ) {
                            let output =
                                <super::AmountType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_due_payable_amount(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(
            mut self,
            reader: &R,
        ) -> Result<super::TradeSettlementHeaderMonetarySummationType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                TradeSettlementHeaderMonetarySummationTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::TradeSettlementHeaderMonetarySummationType {
                line_total_amount: self
                    .line_total_amount
                    .ok_or_else(|| ErrorKind::MissingElement("LineTotalAmount".into()))?,
                charge_total_amount: self.charge_total_amount,
                allowance_total_amount: self.allowance_total_amount,
                tax_basis_total_amount: self
                    .tax_basis_total_amount
                    .ok_or_else(|| ErrorKind::MissingElement("TaxBasisTotalAmount".into()))?,
                tax_total_amount: self.tax_total_amount,
                grand_total_amount: self
                    .grand_total_amount
                    .ok_or_else(|| ErrorKind::MissingElement("GrandTotalAmount".into()))?,
                total_prepaid_amount: self.total_prepaid_amount,
                due_payable_amount: self
                    .due_payable_amount
                    .ok_or_else(|| ErrorKind::MissingElement("DuePayableAmount".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub struct TradeAccountingAccountTypeDeserializer {
        id: Option<super::IdType>,
        state: Box<TradeAccountingAccountTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum TradeAccountingAccountTypeDeserializerState {
        Init__,
        Id(Option<<super::IdType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl TradeAccountingAccountTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
            }
            Ok(Self {
                id: None,
                state: Box::new(TradeAccountingAccountTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: TradeAccountingAccountTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use TradeAccountingAccountTypeDeserializerState as S;
            match state {
                S::Id(Some(deserializer)) => self.store_id(deserializer.finish(reader)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_id(&mut self, value: super::IdType) -> Result<(), Error> {
            if self.id.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"ID")))?;
            }
            self.id = Some(value);
            Ok(())
        }
        fn handle_id<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::IdType>,
            fallback: &mut Option<TradeAccountingAccountTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.id.is_some() {
                    fallback.get_or_insert(TradeAccountingAccountTypeDeserializerState::Id(None));
                    *self.state = TradeAccountingAccountTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = TradeAccountingAccountTypeDeserializerState::Id(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_id(data)?;
                    *self.state = TradeAccountingAccountTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                TradeAccountingAccountTypeDeserializerState::Id(Some(deserializer)),
                            );
                            *self.state = TradeAccountingAccountTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                TradeAccountingAccountTypeDeserializerState::Id(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::TradeAccountingAccountType>
        for TradeAccountingAccountTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TradeAccountingAccountType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TradeAccountingAccountType>
        where
            R: DeserializeReader,
        {
            use TradeAccountingAccountTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Id(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_id(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = TradeAccountingAccountTypeDeserializerState::Id(None);
                        event
                    }
                    (S::Id(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_RAM), b"ID") {
                            let output = <super::IdType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            match self.handle_id(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::TradeAccountingAccountType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                TradeAccountingAccountTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::TradeAccountingAccountType {
                id: self
                    .id
                    .ok_or_else(|| ErrorKind::MissingElement("ID".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub struct LegalOrganizationTypeDeserializer {
        id: Option<super::IdType>,
        trading_business_name: Option<super::TextType>,
        state: Box<LegalOrganizationTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum LegalOrganizationTypeDeserializerState {
        Init__,
        Id(Option<<super::IdType as WithDeserializer>::Deserializer>),
        TradingBusinessName(Option<<super::TextType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl LegalOrganizationTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
            }
            Ok(Self {
                id: None,
                trading_business_name: None,
                state: Box::new(LegalOrganizationTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: LegalOrganizationTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use LegalOrganizationTypeDeserializerState as S;
            match state {
                S::Id(Some(deserializer)) => self.store_id(deserializer.finish(reader)?)?,
                S::TradingBusinessName(Some(deserializer)) => {
                    self.store_trading_business_name(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_id(&mut self, value: super::IdType) -> Result<(), Error> {
            if self.id.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"ID")))?;
            }
            self.id = Some(value);
            Ok(())
        }
        fn store_trading_business_name(&mut self, value: super::TextType) -> Result<(), Error> {
            if self.trading_business_name.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"TradingBusinessName",
                )))?;
            }
            self.trading_business_name = Some(value);
            Ok(())
        }
        fn handle_id<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::IdType>,
            fallback: &mut Option<LegalOrganizationTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(LegalOrganizationTypeDeserializerState::Id(None));
                *self.state = LegalOrganizationTypeDeserializerState::TradingBusinessName(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_id(data)?;
                    *self.state = LegalOrganizationTypeDeserializerState::TradingBusinessName(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(LegalOrganizationTypeDeserializerState::Id(
                                Some(deserializer),
                            ));
                            *self.state =
                                LegalOrganizationTypeDeserializerState::TradingBusinessName(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                LegalOrganizationTypeDeserializerState::Id(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_trading_business_name<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::TextType>,
            fallback: &mut Option<LegalOrganizationTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(
                    LegalOrganizationTypeDeserializerState::TradingBusinessName(None),
                );
                *self.state = LegalOrganizationTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_trading_business_name(data)?;
                    *self.state = LegalOrganizationTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                LegalOrganizationTypeDeserializerState::TradingBusinessName(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = LegalOrganizationTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                LegalOrganizationTypeDeserializerState::TradingBusinessName(Some(
                                    deserializer,
                                ));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::LegalOrganizationType> for LegalOrganizationTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::LegalOrganizationType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::LegalOrganizationType>
        where
            R: DeserializeReader,
        {
            use LegalOrganizationTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Id(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_id(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::TradingBusinessName(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_trading_business_name(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = LegalOrganizationTypeDeserializerState::Id(None);
                        event
                    }
                    (S::Id(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_RAM), b"ID") {
                            let output = <super::IdType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            match self.handle_id(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::TradingBusinessName(None);
                            event
                        }
                    }
                    (S::TradingBusinessName(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"TradingBusinessName",
                        ) {
                            let output = <super::TextType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            match self.handle_trading_business_name(
                                reader,
                                output,
                                &mut fallback,
                            )? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::LegalOrganizationType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                LegalOrganizationTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::LegalOrganizationType {
                id: self.id,
                trading_business_name: self.trading_business_name,
            })
        }
    }
    #[derive(Debug)]
    pub struct TradeAddressTypeDeserializer {
        postcode_code: Option<super::CodeType>,
        line_one: Option<super::TextType>,
        line_two: Option<super::TextType>,
        line_three: Option<super::TextType>,
        city_name: Option<super::TextType>,
        country_id: Option<super::CountryIdType>,
        country_sub_division_name: Option<super::TextType>,
        state: Box<TradeAddressTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum TradeAddressTypeDeserializerState {
        Init__,
        PostcodeCode(Option<<super::CodeType as WithDeserializer>::Deserializer>),
        LineOne(Option<<super::TextType as WithDeserializer>::Deserializer>),
        LineTwo(Option<<super::TextType as WithDeserializer>::Deserializer>),
        LineThree(Option<<super::TextType as WithDeserializer>::Deserializer>),
        CityName(Option<<super::TextType as WithDeserializer>::Deserializer>),
        CountryId(Option<<super::CountryIdType as WithDeserializer>::Deserializer>),
        CountrySubDivisionName(Option<<super::TextType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl TradeAddressTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
            }
            Ok(Self {
                postcode_code: None,
                line_one: None,
                line_two: None,
                line_three: None,
                city_name: None,
                country_id: None,
                country_sub_division_name: None,
                state: Box::new(TradeAddressTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: TradeAddressTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use TradeAddressTypeDeserializerState as S;
            match state {
                S::PostcodeCode(Some(deserializer)) => {
                    self.store_postcode_code(deserializer.finish(reader)?)?
                }
                S::LineOne(Some(deserializer)) => {
                    self.store_line_one(deserializer.finish(reader)?)?
                }
                S::LineTwo(Some(deserializer)) => {
                    self.store_line_two(deserializer.finish(reader)?)?
                }
                S::LineThree(Some(deserializer)) => {
                    self.store_line_three(deserializer.finish(reader)?)?
                }
                S::CityName(Some(deserializer)) => {
                    self.store_city_name(deserializer.finish(reader)?)?
                }
                S::CountryId(Some(deserializer)) => {
                    self.store_country_id(deserializer.finish(reader)?)?
                }
                S::CountrySubDivisionName(Some(deserializer)) => {
                    self.store_country_sub_division_name(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_postcode_code(&mut self, value: super::CodeType) -> Result<(), Error> {
            if self.postcode_code.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"PostcodeCode",
                )))?;
            }
            self.postcode_code = Some(value);
            Ok(())
        }
        fn store_line_one(&mut self, value: super::TextType) -> Result<(), Error> {
            if self.line_one.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"LineOne",
                )))?;
            }
            self.line_one = Some(value);
            Ok(())
        }
        fn store_line_two(&mut self, value: super::TextType) -> Result<(), Error> {
            if self.line_two.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"LineTwo",
                )))?;
            }
            self.line_two = Some(value);
            Ok(())
        }
        fn store_line_three(&mut self, value: super::TextType) -> Result<(), Error> {
            if self.line_three.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"LineThree",
                )))?;
            }
            self.line_three = Some(value);
            Ok(())
        }
        fn store_city_name(&mut self, value: super::TextType) -> Result<(), Error> {
            if self.city_name.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"CityName",
                )))?;
            }
            self.city_name = Some(value);
            Ok(())
        }
        fn store_country_id(&mut self, value: super::CountryIdType) -> Result<(), Error> {
            if self.country_id.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"CountryID",
                )))?;
            }
            self.country_id = Some(value);
            Ok(())
        }
        fn store_country_sub_division_name(&mut self, value: super::TextType) -> Result<(), Error> {
            if self.country_sub_division_name.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"CountrySubDivisionName",
                )))?;
            }
            self.country_sub_division_name = Some(value);
            Ok(())
        }
        fn handle_postcode_code<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::CodeType>,
            fallback: &mut Option<TradeAddressTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(TradeAddressTypeDeserializerState::PostcodeCode(None));
                *self.state = TradeAddressTypeDeserializerState::LineOne(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_postcode_code(data)?;
                    *self.state = TradeAddressTypeDeserializerState::LineOne(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                TradeAddressTypeDeserializerState::PostcodeCode(Some(deserializer)),
                            );
                            *self.state = TradeAddressTypeDeserializerState::LineOne(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                TradeAddressTypeDeserializerState::PostcodeCode(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_line_one<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::TextType>,
            fallback: &mut Option<TradeAddressTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(TradeAddressTypeDeserializerState::LineOne(None));
                *self.state = TradeAddressTypeDeserializerState::LineTwo(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_line_one(data)?;
                    *self.state = TradeAddressTypeDeserializerState::LineTwo(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(TradeAddressTypeDeserializerState::LineOne(
                                Some(deserializer),
                            ));
                            *self.state = TradeAddressTypeDeserializerState::LineTwo(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                TradeAddressTypeDeserializerState::LineOne(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_line_two<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::TextType>,
            fallback: &mut Option<TradeAddressTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(TradeAddressTypeDeserializerState::LineTwo(None));
                *self.state = TradeAddressTypeDeserializerState::LineThree(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_line_two(data)?;
                    *self.state = TradeAddressTypeDeserializerState::LineThree(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(TradeAddressTypeDeserializerState::LineTwo(
                                Some(deserializer),
                            ));
                            *self.state = TradeAddressTypeDeserializerState::LineThree(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                TradeAddressTypeDeserializerState::LineTwo(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_line_three<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::TextType>,
            fallback: &mut Option<TradeAddressTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(TradeAddressTypeDeserializerState::LineThree(None));
                *self.state = TradeAddressTypeDeserializerState::CityName(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_line_three(data)?;
                    *self.state = TradeAddressTypeDeserializerState::CityName(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(TradeAddressTypeDeserializerState::LineThree(
                                Some(deserializer),
                            ));
                            *self.state = TradeAddressTypeDeserializerState::CityName(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                TradeAddressTypeDeserializerState::LineThree(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_city_name<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::TextType>,
            fallback: &mut Option<TradeAddressTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(TradeAddressTypeDeserializerState::CityName(None));
                *self.state = TradeAddressTypeDeserializerState::CountryId(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_city_name(data)?;
                    *self.state = TradeAddressTypeDeserializerState::CountryId(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(TradeAddressTypeDeserializerState::CityName(
                                Some(deserializer),
                            ));
                            *self.state = TradeAddressTypeDeserializerState::CountryId(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                TradeAddressTypeDeserializerState::CityName(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_country_id<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::CountryIdType>,
            fallback: &mut Option<TradeAddressTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.country_id.is_some() {
                    fallback.get_or_insert(TradeAddressTypeDeserializerState::CountryId(None));
                    *self.state = TradeAddressTypeDeserializerState::CountrySubDivisionName(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = TradeAddressTypeDeserializerState::CountryId(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_country_id(data)?;
                    *self.state = TradeAddressTypeDeserializerState::CountrySubDivisionName(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(TradeAddressTypeDeserializerState::CountryId(
                                Some(deserializer),
                            ));
                            *self.state =
                                TradeAddressTypeDeserializerState::CountrySubDivisionName(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                TradeAddressTypeDeserializerState::CountryId(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_country_sub_division_name<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::TextType>,
            fallback: &mut Option<TradeAddressTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(TradeAddressTypeDeserializerState::CountrySubDivisionName(
                    None,
                ));
                *self.state = TradeAddressTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_country_sub_division_name(data)?;
                    *self.state = TradeAddressTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                TradeAddressTypeDeserializerState::CountrySubDivisionName(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = TradeAddressTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = TradeAddressTypeDeserializerState::CountrySubDivisionName(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::TradeAddressType> for TradeAddressTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TradeAddressType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TradeAddressType>
        where
            R: DeserializeReader,
        {
            use TradeAddressTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::PostcodeCode(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_postcode_code(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::LineOne(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_line_one(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::LineTwo(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_line_two(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::LineThree(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_line_three(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::CityName(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_city_name(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::CountryId(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_country_id(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::CountrySubDivisionName(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_country_sub_division_name(
                            reader,
                            output,
                            &mut fallback,
                        )? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = TradeAddressTypeDeserializerState::PostcodeCode(None);
                        event
                    }
                    (S::PostcodeCode(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"PostcodeCode",
                        ) {
                            let output = <super::CodeType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            match self.handle_postcode_code(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::LineOne(None);
                            event
                        }
                    }
                    (S::LineOne(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_RAM), b"LineOne") {
                            let output = <super::TextType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            match self.handle_line_one(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::LineTwo(None);
                            event
                        }
                    }
                    (S::LineTwo(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_RAM), b"LineTwo") {
                            let output = <super::TextType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            match self.handle_line_two(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::LineThree(None);
                            event
                        }
                    }
                    (S::LineThree(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_RAM), b"LineThree") {
                            let output = <super::TextType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            match self.handle_line_three(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::CityName(None);
                            event
                        }
                    }
                    (S::CityName(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_RAM), b"CityName") {
                            let output = <super::TextType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            match self.handle_city_name(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::CountryId(None);
                            event
                        }
                    }
                    (S::CountryId(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_RAM), b"CountryID") {
                            let output =
                                <super::CountryIdType as WithDeserializer>::Deserializer::init(
                                    reader, event,
                                )?;
                            match self.handle_country_id(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::CountrySubDivisionName(None);
                            event
                        }
                    }
                    (
                        S::CountrySubDivisionName(None),
                        event @ (Event::Start(_) | Event::Empty(_)),
                    ) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"CountrySubDivisionName",
                        ) {
                            let output = <super::TextType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            match self.handle_country_sub_division_name(
                                reader,
                                output,
                                &mut fallback,
                            )? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::TradeAddressType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                TradeAddressTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::TradeAddressType {
                postcode_code: self.postcode_code,
                line_one: self.line_one,
                line_two: self.line_two,
                line_three: self.line_three,
                city_name: self.city_name,
                country_id: self
                    .country_id
                    .ok_or_else(|| ErrorKind::MissingElement("CountryID".into()))?,
                country_sub_division_name: self.country_sub_division_name,
            })
        }
    }
    #[derive(Debug)]
    pub struct UniversalCommunicationTypeDeserializer {
        uriid: Option<super::IdType>,
        state: Box<UniversalCommunicationTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum UniversalCommunicationTypeDeserializerState {
        Init__,
        Uriid(Option<<super::IdType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl UniversalCommunicationTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
            }
            Ok(Self {
                uriid: None,
                state: Box::new(UniversalCommunicationTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: UniversalCommunicationTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use UniversalCommunicationTypeDeserializerState as S;
            match state {
                S::Uriid(Some(deserializer)) => self.store_uriid(deserializer.finish(reader)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_uriid(&mut self, value: super::IdType) -> Result<(), Error> {
            if self.uriid.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"URIID",
                )))?;
            }
            self.uriid = Some(value);
            Ok(())
        }
        fn handle_uriid<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::IdType>,
            fallback: &mut Option<UniversalCommunicationTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.uriid.is_some() {
                    fallback
                        .get_or_insert(UniversalCommunicationTypeDeserializerState::Uriid(None));
                    *self.state = UniversalCommunicationTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = UniversalCommunicationTypeDeserializerState::Uriid(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_uriid(data)?;
                    *self.state = UniversalCommunicationTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                UniversalCommunicationTypeDeserializerState::Uriid(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = UniversalCommunicationTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = UniversalCommunicationTypeDeserializerState::Uriid(Some(
                                deserializer,
                            ));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::UniversalCommunicationType>
        for UniversalCommunicationTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::UniversalCommunicationType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::UniversalCommunicationType>
        where
            R: DeserializeReader,
        {
            use UniversalCommunicationTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Uriid(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_uriid(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = UniversalCommunicationTypeDeserializerState::Uriid(None);
                        event
                    }
                    (S::Uriid(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_RAM), b"URIID") {
                            let output = <super::IdType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            match self.handle_uriid(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::UniversalCommunicationType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                UniversalCommunicationTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::UniversalCommunicationType {
                uriid: self
                    .uriid
                    .ok_or_else(|| ErrorKind::MissingElement("URIID".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub struct TaxRegistrationTypeDeserializer {
        id: Option<super::IdType>,
        state: Box<TaxRegistrationTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum TaxRegistrationTypeDeserializerState {
        Init__,
        Id(Option<<super::IdType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl TaxRegistrationTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
            }
            Ok(Self {
                id: None,
                state: Box::new(TaxRegistrationTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: TaxRegistrationTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use TaxRegistrationTypeDeserializerState as S;
            match state {
                S::Id(Some(deserializer)) => self.store_id(deserializer.finish(reader)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_id(&mut self, value: super::IdType) -> Result<(), Error> {
            if self.id.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"ID")))?;
            }
            self.id = Some(value);
            Ok(())
        }
        fn handle_id<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::IdType>,
            fallback: &mut Option<TaxRegistrationTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.id.is_some() {
                    fallback.get_or_insert(TaxRegistrationTypeDeserializerState::Id(None));
                    *self.state = TaxRegistrationTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = TaxRegistrationTypeDeserializerState::Id(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_id(data)?;
                    *self.state = TaxRegistrationTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(TaxRegistrationTypeDeserializerState::Id(Some(
                                deserializer,
                            )));
                            *self.state = TaxRegistrationTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                TaxRegistrationTypeDeserializerState::Id(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::TaxRegistrationType> for TaxRegistrationTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TaxRegistrationType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TaxRegistrationType>
        where
            R: DeserializeReader,
        {
            use TaxRegistrationTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Id(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_id(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = TaxRegistrationTypeDeserializerState::Id(None);
                        event
                    }
                    (S::Id(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_RAM), b"ID") {
                            let output = <super::IdType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            match self.handle_id(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::TaxRegistrationType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                TaxRegistrationTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::TaxRegistrationType {
                id: self
                    .id
                    .ok_or_else(|| ErrorKind::MissingElement("ID".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub struct FormattedDateTimeTypeDeserializer {
        date_time_string: Option<super::FormattedDateTimeTypeDateTimeStringType>,
        state: Box<FormattedDateTimeTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum FormattedDateTimeTypeDeserializerState {
        Init__,
        DateTimeString(
            Option<
                <super::FormattedDateTimeTypeDateTimeStringType as WithDeserializer>::Deserializer,
            >,
        ),
        Done__,
        Unknown__,
    }
    impl FormattedDateTimeTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
            }
            Ok(Self {
                date_time_string: None,
                state: Box::new(FormattedDateTimeTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: FormattedDateTimeTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use FormattedDateTimeTypeDeserializerState as S;
            match state {
                S::DateTimeString(Some(deserializer)) => {
                    self.store_date_time_string(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_date_time_string(
            &mut self,
            value: super::FormattedDateTimeTypeDateTimeStringType,
        ) -> Result<(), Error> {
            if self.date_time_string.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"DateTimeString",
                )))?;
            }
            self.date_time_string = Some(value);
            Ok(())
        }
        fn handle_date_time_string<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::FormattedDateTimeTypeDateTimeStringType>,
            fallback: &mut Option<FormattedDateTimeTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.date_time_string.is_some() {
                    fallback.get_or_insert(FormattedDateTimeTypeDeserializerState::DateTimeString(
                        None,
                    ));
                    *self.state = FormattedDateTimeTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = FormattedDateTimeTypeDeserializerState::DateTimeString(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_date_time_string(data)?;
                    *self.state = FormattedDateTimeTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                FormattedDateTimeTypeDeserializerState::DateTimeString(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = FormattedDateTimeTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = FormattedDateTimeTypeDeserializerState::DateTimeString(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::FormattedDateTimeType> for FormattedDateTimeTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::FormattedDateTimeType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::FormattedDateTimeType>
        where
            R: DeserializeReader,
        {
            use FormattedDateTimeTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::DateTimeString(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_date_time_string(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = FormattedDateTimeTypeDeserializerState::DateTimeString(None);
                        event
                    }
                    (S::DateTimeString(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_QDT),
                            b"DateTimeString",
                        ) {
                            let output = < super :: FormattedDateTimeTypeDateTimeStringType as WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_date_time_string(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::FormattedDateTimeType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                FormattedDateTimeTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::FormattedDateTimeType {
                date_time_string: self
                    .date_time_string
                    .ok_or_else(|| ErrorKind::MissingElement("DateTimeString".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub struct PaymentMeansCodeTypeDeserializer {
        content: Option<String>,
        state: Box<PaymentMeansCodeTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum PaymentMeansCodeTypeDeserializerState {
        Init__,
        Content__(<String as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl PaymentMeansCodeTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
            }
            Ok(Self {
                content: None,
                state: Box::new(PaymentMeansCodeTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: PaymentMeansCodeTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let PaymentMeansCodeTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: String) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
        ) -> DeserializerResult<'de, super::PaymentMeansCodeType>
        where
            R: DeserializeReader,
        {
            use PaymentMeansCodeTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::PaymentMeansCodeType> for PaymentMeansCodeTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PaymentMeansCodeType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PaymentMeansCodeType>
        where
            R: DeserializeReader,
        {
            use PaymentMeansCodeTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::PaymentMeansCodeType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                PaymentMeansCodeTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::PaymentMeansCodeType {
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct DebtorFinancialAccountTypeDeserializer {
        ibanid: Option<super::IdType>,
        state: Box<DebtorFinancialAccountTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum DebtorFinancialAccountTypeDeserializerState {
        Init__,
        Ibanid(Option<<super::IdType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl DebtorFinancialAccountTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
            }
            Ok(Self {
                ibanid: None,
                state: Box::new(DebtorFinancialAccountTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: DebtorFinancialAccountTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use DebtorFinancialAccountTypeDeserializerState as S;
            match state {
                S::Ibanid(Some(deserializer)) => self.store_ibanid(deserializer.finish(reader)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_ibanid(&mut self, value: super::IdType) -> Result<(), Error> {
            if self.ibanid.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"IBANID",
                )))?;
            }
            self.ibanid = Some(value);
            Ok(())
        }
        fn handle_ibanid<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::IdType>,
            fallback: &mut Option<DebtorFinancialAccountTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.ibanid.is_some() {
                    fallback
                        .get_or_insert(DebtorFinancialAccountTypeDeserializerState::Ibanid(None));
                    *self.state = DebtorFinancialAccountTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state = DebtorFinancialAccountTypeDeserializerState::Ibanid(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_ibanid(data)?;
                    *self.state = DebtorFinancialAccountTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                DebtorFinancialAccountTypeDeserializerState::Ibanid(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = DebtorFinancialAccountTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = DebtorFinancialAccountTypeDeserializerState::Ibanid(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::DebtorFinancialAccountType>
        for DebtorFinancialAccountTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DebtorFinancialAccountType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DebtorFinancialAccountType>
        where
            R: DeserializeReader,
        {
            use DebtorFinancialAccountTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Ibanid(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_ibanid(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = DebtorFinancialAccountTypeDeserializerState::Ibanid(None);
                        event
                    }
                    (S::Ibanid(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_RAM), b"IBANID") {
                            let output = <super::IdType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            match self.handle_ibanid(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::DebtorFinancialAccountType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                DebtorFinancialAccountTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::DebtorFinancialAccountType {
                ibanid: self
                    .ibanid
                    .ok_or_else(|| ErrorKind::MissingElement("IBANID".into()))?,
            })
        }
    }
    #[derive(Debug)]
    pub struct CreditorFinancialAccountTypeDeserializer {
        ibanid: Option<super::IdType>,
        proprietary_id: Option<super::IdType>,
        state: Box<CreditorFinancialAccountTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum CreditorFinancialAccountTypeDeserializerState {
        Init__,
        Ibanid(Option<<super::IdType as WithDeserializer>::Deserializer>),
        ProprietaryId(Option<<super::IdType as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl CreditorFinancialAccountTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
            }
            Ok(Self {
                ibanid: None,
                proprietary_id: None,
                state: Box::new(CreditorFinancialAccountTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: CreditorFinancialAccountTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use CreditorFinancialAccountTypeDeserializerState as S;
            match state {
                S::Ibanid(Some(deserializer)) => self.store_ibanid(deserializer.finish(reader)?)?,
                S::ProprietaryId(Some(deserializer)) => {
                    self.store_proprietary_id(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_ibanid(&mut self, value: super::IdType) -> Result<(), Error> {
            if self.ibanid.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"IBANID",
                )))?;
            }
            self.ibanid = Some(value);
            Ok(())
        }
        fn store_proprietary_id(&mut self, value: super::IdType) -> Result<(), Error> {
            if self.proprietary_id.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"ProprietaryID",
                )))?;
            }
            self.proprietary_id = Some(value);
            Ok(())
        }
        fn handle_ibanid<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::IdType>,
            fallback: &mut Option<CreditorFinancialAccountTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(CreditorFinancialAccountTypeDeserializerState::Ibanid(None));
                *self.state = CreditorFinancialAccountTypeDeserializerState::ProprietaryId(None);
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_ibanid(data)?;
                    *self.state =
                        CreditorFinancialAccountTypeDeserializerState::ProprietaryId(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                CreditorFinancialAccountTypeDeserializerState::Ibanid(Some(
                                    deserializer,
                                )),
                            );
                            *self.state =
                                CreditorFinancialAccountTypeDeserializerState::ProprietaryId(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state = CreditorFinancialAccountTypeDeserializerState::Ibanid(
                                Some(deserializer),
                            );
                        }
                    }
                    ret
                }
            })
        }
        fn handle_proprietary_id<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::IdType>,
            fallback: &mut Option<CreditorFinancialAccountTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(
                    CreditorFinancialAccountTypeDeserializerState::ProprietaryId(None),
                );
                *self.state = CreditorFinancialAccountTypeDeserializerState::Done__;
                return Ok(ElementHandlerOutput::from_event(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_proprietary_id(data)?;
                    *self.state = CreditorFinancialAccountTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(
                                CreditorFinancialAccountTypeDeserializerState::ProprietaryId(Some(
                                    deserializer,
                                )),
                            );
                            *self.state = CreditorFinancialAccountTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state =
                                CreditorFinancialAccountTypeDeserializerState::ProprietaryId(Some(
                                    deserializer,
                                ));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::CreditorFinancialAccountType>
        for CreditorFinancialAccountTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::CreditorFinancialAccountType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::CreditorFinancialAccountType>
        where
            R: DeserializeReader,
        {
            use CreditorFinancialAccountTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Ibanid(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_ibanid(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::ProprietaryId(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_proprietary_id(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state = CreditorFinancialAccountTypeDeserializerState::Ibanid(None);
                        event
                    }
                    (S::Ibanid(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(&event, Some(&super::NS_RAM), b"IBANID") {
                            let output = <super::IdType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            match self.handle_ibanid(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::ProprietaryId(None);
                            event
                        }
                    }
                    (S::ProprietaryId(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        if reader.check_start_tag_name(
                            &event,
                            Some(&super::NS_RAM),
                            b"ProprietaryID",
                        ) {
                            let output = <super::IdType as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                            match self.handle_proprietary_id(reader, output, &mut fallback)? {
                                ElementHandlerOutput::Continue { event, allow_any } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ElementHandlerOutput::Break { event, allow_any } => {
                                    break (event, allow_any)
                                }
                            }
                        } else {
                            *self.state = S::Done__;
                            event
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::CreditorFinancialAccountType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                CreditorFinancialAccountTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::CreditorFinancialAccountType {
                ibanid: self.ibanid,
                proprietary_id: self.proprietary_id,
            })
        }
    }
    #[derive(Debug)]
    pub struct AmountTypeDeserializer {
        currency_id: Option<String>,
        content: Option<f64>,
        state: Box<AmountTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum AmountTypeDeserializerState {
        Init__,
        Content__(<f64 as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl AmountTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut currency_id: Option<String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_UDT),
                    Some(b"currencyID")
                ) {
                    reader.read_attrib(&mut currency_id, b"currencyID", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib(attrib)?;
                }
            }
            Ok(Self {
                currency_id: currency_id,
                content: None,
                state: Box::new(AmountTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: AmountTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let AmountTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: f64) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, f64>,
        ) -> DeserializerResult<'de, super::AmountType>
        where
            R: DeserializeReader,
        {
            use AmountTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::AmountType> for AmountTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::AmountType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AmountType>
        where
            R: DeserializeReader,
        {
            use AmountTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::AmountType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, AmountTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::AmountType {
                currency_id: self.currency_id,
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct TaxTypeCodeTypeDeserializer {
        content: Option<String>,
        state: Box<TaxTypeCodeTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum TaxTypeCodeTypeDeserializerState {
        Init__,
        Content__(<String as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl TaxTypeCodeTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
            }
            Ok(Self {
                content: None,
                state: Box::new(TaxTypeCodeTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: TaxTypeCodeTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let TaxTypeCodeTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: String) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
        ) -> DeserializerResult<'de, super::TaxTypeCodeType>
        where
            R: DeserializeReader,
        {
            use TaxTypeCodeTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::TaxTypeCodeType> for TaxTypeCodeTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::TaxTypeCodeType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TaxTypeCodeType>
        where
            R: DeserializeReader,
        {
            use TaxTypeCodeTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::TaxTypeCodeType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                TaxTypeCodeTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::TaxTypeCodeType {
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct TaxCategoryCodeTypeDeserializer {
        content: Option<String>,
        state: Box<TaxCategoryCodeTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum TaxCategoryCodeTypeDeserializerState {
        Init__,
        Content__(<String as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl TaxCategoryCodeTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
            }
            Ok(Self {
                content: None,
                state: Box::new(TaxCategoryCodeTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: TaxCategoryCodeTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let TaxCategoryCodeTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: String) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
        ) -> DeserializerResult<'de, super::TaxCategoryCodeType>
        where
            R: DeserializeReader,
        {
            use TaxCategoryCodeTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::TaxCategoryCodeType> for TaxCategoryCodeTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TaxCategoryCodeType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TaxCategoryCodeType>
        where
            R: DeserializeReader,
        {
            use TaxCategoryCodeTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::TaxCategoryCodeType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                TaxCategoryCodeTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::TaxCategoryCodeType {
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct TimeReferenceCodeTypeDeserializer {
        content: Option<String>,
        state: Box<TimeReferenceCodeTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum TimeReferenceCodeTypeDeserializerState {
        Init__,
        Content__(<String as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl TimeReferenceCodeTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
            }
            Ok(Self {
                content: None,
                state: Box::new(TimeReferenceCodeTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: TimeReferenceCodeTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let TimeReferenceCodeTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: String) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
        ) -> DeserializerResult<'de, super::TimeReferenceCodeType>
        where
            R: DeserializeReader,
        {
            use TimeReferenceCodeTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::TimeReferenceCodeType> for TimeReferenceCodeTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TimeReferenceCodeType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::TimeReferenceCodeType>
        where
            R: DeserializeReader,
        {
            use TimeReferenceCodeTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::TimeReferenceCodeType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                TimeReferenceCodeTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::TimeReferenceCodeType {
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct PercentTypeDeserializer {
        content: Option<f64>,
        state: Box<PercentTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum PercentTypeDeserializerState {
        Init__,
        Content__(<f64 as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl PercentTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
            }
            Ok(Self {
                content: None,
                state: Box::new(PercentTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: PercentTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let PercentTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: f64) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, f64>,
        ) -> DeserializerResult<'de, super::PercentType>
        where
            R: DeserializeReader,
        {
            use PercentTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::PercentType> for PercentTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::PercentType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::PercentType>
        where
            R: DeserializeReader,
        {
            use PercentTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::PercentType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, PercentTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::PercentType {
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct IndicatorTypeDeserializer {
        content: Option<super::IndicatorTypeContent>,
        state: Box<IndicatorTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum IndicatorTypeDeserializerState {
        Init__,
        Next__,
        Content__(<super::IndicatorTypeContent as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl IndicatorTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
            }
            Ok(Self {
                content: None,
                state: Box::new(IndicatorTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: IndicatorTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let IndicatorTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: super::IndicatorTypeContent) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, super::IndicatorTypeContent>,
            fallback: &mut Option<IndicatorTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = fallback
                    .take()
                    .unwrap_or(IndicatorTypeDeserializerState::Next__);
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state = IndicatorTypeDeserializerState::Next__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = IndicatorTypeDeserializerState::Content__(deserializer);
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::IndicatorType> for IndicatorTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::IndicatorType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::IndicatorType>
        where
            R: DeserializeReader,
        {
            use IndicatorTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Content__(deserializer), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (_, Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        let output =
                            <super::IndicatorTypeContent as WithDeserializer>::Deserializer::init(
                                reader, event,
                            )?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = DeserializerArtifact::Deserializer(self);
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::IndicatorType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, IndicatorTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::IndicatorType {
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct IndicatorTypeContentDeserializer {
        state: Box<IndicatorTypeContentDeserializerState>,
    }
    #[derive(Debug)]
    pub enum IndicatorTypeContentDeserializerState {
        Init__,
        Indicator(
            Option<bool>,
            Option<<bool as WithDeserializer>::Deserializer>,
        ),
        Done__(super::IndicatorTypeContent),
        Unknown__,
    }
    impl IndicatorTypeContentDeserializer {
        fn find_suitable<'de, R>(
            &mut self,
            reader: &R,
            event: Event<'de>,
            fallback: &mut Option<IndicatorTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                *self.state = fallback
                    .take()
                    .unwrap_or(IndicatorTypeContentDeserializerState::Init__);
                return Ok(ElementHandlerOutput::return_to_parent(event, false));
            };
            if matches!(
                reader.resolve_local_name(x.name(), &super::NS_UDT),
                Some(b"Indicator")
            ) {
                let output = <bool as WithDeserializer>::Deserializer::init(reader, event)?;
                return self.handle_indicator(reader, Default::default(), output, &mut *fallback);
            }
            *self.state = fallback
                .take()
                .unwrap_or(IndicatorTypeContentDeserializerState::Init__);
            Ok(ElementHandlerOutput::return_to_parent(event, false))
        }
        fn finish_state<R>(
            reader: &R,
            state: IndicatorTypeContentDeserializerState,
        ) -> Result<super::IndicatorTypeContent, Error>
        where
            R: DeserializeReader,
        {
            use IndicatorTypeContentDeserializerState as S;
            match state {
                S::Init__ => Err(ErrorKind::MissingContent.into()),
                S::Indicator(mut values, deserializer) => {
                    if let Some(deserializer) = deserializer {
                        let value = deserializer.finish(reader)?;
                        Self::store_indicator(&mut values, value)?;
                    }
                    Ok(super::IndicatorTypeContent::Indicator(values.ok_or_else(
                        || ErrorKind::MissingElement("Indicator".into()),
                    )?))
                }
                S::Done__(data) => Ok(data),
                S::Unknown__ => unreachable!(),
            }
        }
        fn store_indicator(values: &mut Option<bool>, value: bool) -> Result<(), Error> {
            if values.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Indicator",
                )))?;
            }
            *values = Some(value);
            Ok(())
        }
        fn handle_indicator<'de, R>(
            &mut self,
            reader: &R,
            mut values: Option<bool>,
            output: DeserializerOutput<'de, bool>,
            fallback: &mut Option<IndicatorTypeContentDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state = match fallback.take() {
                    None => IndicatorTypeContentDeserializerState::Init__,
                    Some(IndicatorTypeContentDeserializerState::Indicator(
                        _,
                        Some(deserializer),
                    )) => {
                        IndicatorTypeContentDeserializerState::Indicator(values, Some(deserializer))
                    }
                    _ => unreachable!(),
                };
                return Ok(ElementHandlerOutput::break_(event, allow_any));
            }
            match fallback.take() {
                None => (),
                Some(IndicatorTypeContentDeserializerState::Indicator(_, Some(deserializer))) => {
                    let data = deserializer.finish(reader)?;
                    Self::store_indicator(&mut values, data)?;
                }
                Some(_) => unreachable!(),
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    Self::store_indicator(&mut values, data)?;
                    let data = Self::finish_state(
                        reader,
                        IndicatorTypeContentDeserializerState::Indicator(values, None),
                    )?;
                    *self.state = IndicatorTypeContentDeserializerState::Done__(data);
                    ElementHandlerOutput::Break { event, allow_any }
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = IndicatorTypeContentDeserializerState::Indicator(
                        values,
                        Some(deserializer),
                    );
                    ElementHandlerOutput::from_event_end(event, allow_any)
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::IndicatorTypeContent> for IndicatorTypeContentDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::IndicatorTypeContent>
        where
            R: DeserializeReader,
        {
            let deserializer = Self {
                state: Box::new(IndicatorTypeContentDeserializerState::Init__),
            };
            let mut output = deserializer.next(reader, event)?;
            output.artifact = match output.artifact {
                DeserializerArtifact::Deserializer(x)
                    if matches!(&*x.state, IndicatorTypeContentDeserializerState::Init__) =>
                {
                    DeserializerArtifact::None
                }
                artifact => artifact,
            };
            Ok(output)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::IndicatorTypeContent>
        where
            R: DeserializeReader,
        {
            use IndicatorTypeContentDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state, S::Unknown__);
                event = match (state, event) {
                    (S::Indicator(values, Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_indicator(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (state, event @ Event::End(_)) => {
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(Self::finish_state(
                                reader, state,
                            )?),
                            event: DeserializerEvent::Continue(event),
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => match self.find_suitable(reader, event, &mut fallback)? {
                        ElementHandlerOutput::Break { event, allow_any } => {
                            break (event, allow_any)
                        }
                        ElementHandlerOutput::Continue { event, .. } => event,
                    },
                    (S::Indicator(values, None), event) => {
                        let output = <bool as WithDeserializer>::Deserializer::init(reader, event)?;
                        match self.handle_indicator(reader, values, output, &mut fallback)? {
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                            ElementHandlerOutput::Continue { event, .. } => event,
                        }
                    }
                    (s @ S::Done__(_), event) => {
                        *self.state = s;
                        break (DeserializerEvent::Continue(event), false);
                    }
                    (S::Unknown__, _) => unreachable!(),
                }
            };
            let artifact = if matches!(&*self.state, S::Done__(_)) {
                DeserializerArtifact::Data(self.finish(reader)?)
            } else {
                DeserializerArtifact::Deserializer(self)
            };
            Ok(DeserializerOutput {
                artifact,
                event,
                allow_any,
            })
        }
        fn finish<R>(self, reader: &R) -> Result<super::IndicatorTypeContent, Error>
        where
            R: DeserializeReader,
        {
            Self::finish_state(reader, *self.state)
        }
    }
    #[derive(Debug)]
    pub struct AllowanceChargeReasonCodeTypeDeserializer {
        content: Option<String>,
        state: Box<AllowanceChargeReasonCodeTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum AllowanceChargeReasonCodeTypeDeserializerState {
        Init__,
        Content__(<String as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl AllowanceChargeReasonCodeTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
            }
            Ok(Self {
                content: None,
                state: Box::new(AllowanceChargeReasonCodeTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: AllowanceChargeReasonCodeTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let AllowanceChargeReasonCodeTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: String) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
        ) -> DeserializerResult<'de, super::AllowanceChargeReasonCodeType>
        where
            R: DeserializeReader,
        {
            use AllowanceChargeReasonCodeTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::AllowanceChargeReasonCodeType>
        for AllowanceChargeReasonCodeTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AllowanceChargeReasonCodeType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::AllowanceChargeReasonCodeType>
        where
            R: DeserializeReader,
        {
            use AllowanceChargeReasonCodeTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::AllowanceChargeReasonCodeType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                AllowanceChargeReasonCodeTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::AllowanceChargeReasonCodeType {
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct CountryIdTypeDeserializer {
        content: Option<String>,
        state: Box<CountryIdTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum CountryIdTypeDeserializerState {
        Init__,
        Content__(<String as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl CountryIdTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib(attrib)?;
            }
            Ok(Self {
                content: None,
                state: Box::new(CountryIdTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: CountryIdTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let CountryIdTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: String) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
        ) -> DeserializerResult<'de, super::CountryIdType>
        where
            R: DeserializeReader,
        {
            use CountryIdTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::CountryIdType> for CountryIdTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::CountryIdType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::CountryIdType>
        where
            R: DeserializeReader,
        {
            use CountryIdTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::CountryIdType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state, CountryIdTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::CountryIdType {
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct FormattedDateTimeTypeDateTimeStringTypeDeserializer {
        format: String,
        content: Option<String>,
        state: Box<FormattedDateTimeTypeDateTimeStringTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum FormattedDateTimeTypeDateTimeStringTypeDeserializerState {
        Init__,
        Content__(<String as WithDeserializer>::Deserializer),
        Unknown__,
    }
    impl FormattedDateTimeTypeDateTimeStringTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            let mut format: Option<String> = None;
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    reader.resolve_local_name(attrib.key, &super::NS_QDT),
                    Some(b"format")
                ) {
                    reader.read_attrib(&mut format, b"format", &attrib.value)?;
                } else {
                    reader.raise_unexpected_attrib(attrib)?;
                }
            }
            Ok(Self {
                format: format.ok_or_else(|| {
                    reader.map_error(ErrorKind::MissingAttribute("format".into()))
                })?,
                content: None,
                state: Box::new(FormattedDateTimeTypeDateTimeStringTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: FormattedDateTimeTypeDateTimeStringTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            if let FormattedDateTimeTypeDateTimeStringTypeDeserializerState::Content__(
                deserializer,
            ) = state
            {
                self.store_content(deserializer.finish(reader)?)?;
            }
            Ok(())
        }
        fn store_content(&mut self, value: String) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de, R>(
            mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
        ) -> DeserializerResult<'de, super::FormattedDateTimeTypeDateTimeStringType>
        where
            R: DeserializeReader,
        {
            use FormattedDateTimeTypeDateTimeStringTypeDeserializerState as S;
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                DeserializerArtifact::None => Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event,
                    allow_any,
                }),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(reader)?;
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state = S::Content__(deserializer);
                    Ok(DeserializerOutput {
                        artifact: DeserializerArtifact::Deserializer(self),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> Deserializer<'de, super::FormattedDateTimeTypeDateTimeStringType>
        for FormattedDateTimeTypeDateTimeStringTypeDeserializer
    {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::FormattedDateTimeTypeDateTimeStringType>
        where
            R: DeserializeReader,
        {
            let (Event::Start(x) | Event::Empty(x)) = &event else {
                return Ok(DeserializerOutput {
                    artifact: DeserializerArtifact::None,
                    event: DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(reader, x)?.next(reader, event)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::FormattedDateTimeTypeDateTimeStringType>
        where
            R: DeserializeReader,
        {
            use FormattedDateTimeTypeDateTimeStringTypeDeserializerState as S;
            match replace(&mut *self.state, S::Unknown__) {
                S::Init__ => {
                    let output = ContentDeserializer::init(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(reader, event)?;
                    self.handle_content(reader, output)
                }
                S::Unknown__ => unreachable!(),
            }
        }
        fn finish<R>(
            mut self,
            reader: &R,
        ) -> Result<super::FormattedDateTimeTypeDateTimeStringType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state,
                FormattedDateTimeTypeDateTimeStringTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::FormattedDateTimeTypeDateTimeStringType {
                format: self.format,
                content: self.content.ok_or_else(|| ErrorKind::MissingContent)?,
            })
        }
    }
}
pub mod quick_xml_serialize {
    use core::iter::Iterator;
    use xsd_parser::quick_xml::{
        write_attrib, write_attrib_opt, BytesEnd, BytesStart, Error, Event, IterSerializer,
        WithSerializer,
    };
    #[derive(Debug)]
    pub struct CrossIndustryInvoiceTypeSerializer<'ser> {
        pub(super) value: &'ser super::CrossIndustryInvoiceType,
        pub(super) state: Box<CrossIndustryInvoiceTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum CrossIndustryInvoiceTypeSerializerState<'ser> {
        Init__,
        ExchangedDocumentContext(
            <super::ExchangedDocumentContextType as WithSerializer>::Serializer<'ser>,
        ),
        ExchangedDocument(<super::ExchangedDocumentType as WithSerializer>::Serializer<'ser>),
        SupplyChainTradeTransaction(
            <super::SupplyChainTradeTransactionType as WithSerializer>::Serializer<'ser>,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> CrossIndustryInvoiceTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    CrossIndustryInvoiceTypeSerializerState::Init__ => {
                        *self.state =
                            CrossIndustryInvoiceTypeSerializerState::ExchangedDocumentContext(
                                WithSerializer::serializer(
                                    &self.value.exchanged_document_context,
                                    Some("rsm:ExchangedDocumentContext"),
                                    false,
                                )?,
                            );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:rsm"[..], &super::NS_RSM[..]));
                            bytes.push_attribute((&b"xmlns:qdt"[..], &super::NS_QDT[..]));
                            bytes.push_attribute((&b"xmlns:ram"[..], &super::NS_RAM[..]));
                            bytes.push_attribute((&b"xmlns:udt"[..], &super::NS_UDT[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    CrossIndustryInvoiceTypeSerializerState::ExchangedDocumentContext(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state =
                                    CrossIndustryInvoiceTypeSerializerState::ExchangedDocument(
                                        WithSerializer::serializer(
                                            &self.value.exchanged_document,
                                            Some("rsm:ExchangedDocument"),
                                            false,
                                        )?,
                                    )
                            }
                        }
                    }
                    CrossIndustryInvoiceTypeSerializerState::ExchangedDocument(x) => match x
                        .next()
                        .transpose()?
                    {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state =
                                CrossIndustryInvoiceTypeSerializerState::SupplyChainTradeTransaction(
                                    WithSerializer::serializer(
                                        &self.value.supply_chain_trade_transaction,
                                        Some("rsm:SupplyChainTradeTransaction"),
                                        false,
                                    )?,
                                )
                        }
                    },
                    CrossIndustryInvoiceTypeSerializerState::SupplyChainTradeTransaction(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = CrossIndustryInvoiceTypeSerializerState::End__,
                        }
                    }
                    CrossIndustryInvoiceTypeSerializerState::End__ => {
                        *self.state = CrossIndustryInvoiceTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    CrossIndustryInvoiceTypeSerializerState::Done__ => return Ok(None),
                    CrossIndustryInvoiceTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for CrossIndustryInvoiceTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = CrossIndustryInvoiceTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct ExchangedDocumentContextTypeSerializer<'ser> {
        pub(super) value: &'ser super::ExchangedDocumentContextType,
        pub(super) state: Box<ExchangedDocumentContextTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum ExchangedDocumentContextTypeSerializerState<'ser> {
        Init__,
        BusinessProcessSpecifiedDocumentContextParameter(
            IterSerializer<
                'ser,
                Option<&'ser super::DocumentContextParameterType>,
                super::DocumentContextParameterType,
            >,
        ),
        GuidelineSpecifiedDocumentContextParameter(
            <super::DocumentContextParameterType as WithSerializer>::Serializer<'ser>,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ExchangedDocumentContextTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match & mut * self . state { ExchangedDocumentContextTypeSerializerState :: Init__ => { * self . state = ExchangedDocumentContextTypeSerializerState :: BusinessProcessSpecifiedDocumentContextParameter (IterSerializer :: new (self . value . business_process_specified_document_context_parameter . as_ref () , Some ("ram:BusinessProcessSpecifiedDocumentContextParameter") , false)) ; let mut bytes = BytesStart :: new (self . name) ; if self . is_root { bytes . push_attribute ((& b"xmlns:rsm" [..] , & super :: NS_RSM [..])) ; bytes . push_attribute ((& b"xmlns:qdt" [..] , & super :: NS_QDT [..])) ; bytes . push_attribute ((& b"xmlns:ram" [..] , & super :: NS_RAM [..])) ; bytes . push_attribute ((& b"xmlns:udt" [..] , & super :: NS_UDT [..])) ; } return Ok (Some (Event :: Start (bytes))) } ExchangedDocumentContextTypeSerializerState :: BusinessProcessSpecifiedDocumentContextParameter (x) => match x . next () . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = ExchangedDocumentContextTypeSerializerState :: GuidelineSpecifiedDocumentContextParameter (WithSerializer :: serializer (& self . value . guideline_specified_document_context_parameter , Some ("ram:GuidelineSpecifiedDocumentContextParameter") , false) ?) , } ExchangedDocumentContextTypeSerializerState :: GuidelineSpecifiedDocumentContextParameter (x) => match x . next () . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = ExchangedDocumentContextTypeSerializerState :: End__ , } ExchangedDocumentContextTypeSerializerState :: End__ => { * self . state = ExchangedDocumentContextTypeSerializerState :: Done__ ; return Ok (Some (Event :: End (BytesEnd :: new (self . name)))) ; } ExchangedDocumentContextTypeSerializerState :: Done__ => return Ok (None) , ExchangedDocumentContextTypeSerializerState :: Phantom__ (_) => unreachable ! () , }
            }
        }
    }
    impl<'ser> Iterator for ExchangedDocumentContextTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ExchangedDocumentContextTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct ExchangedDocumentTypeSerializer<'ser> {
        pub(super) value: &'ser super::ExchangedDocumentType,
        pub(super) state: Box<ExchangedDocumentTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum ExchangedDocumentTypeSerializerState<'ser> {
        Init__,
        Id(<super::IdType as WithSerializer>::Serializer<'ser>),
        TypeCode(<super::DocumentCodeType as WithSerializer>::Serializer<'ser>),
        IssueDateTime(<super::DateTimeType as WithSerializer>::Serializer<'ser>),
        IncludedNote(IterSerializer<'ser, &'ser [super::NoteType], super::NoteType>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ExchangedDocumentTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ExchangedDocumentTypeSerializerState::Init__ => {
                        *self.state = ExchangedDocumentTypeSerializerState::Id(
                            WithSerializer::serializer(&self.value.id, Some("ram:ID"), false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:rsm"[..], &super::NS_RSM[..]));
                            bytes.push_attribute((&b"xmlns:qdt"[..], &super::NS_QDT[..]));
                            bytes.push_attribute((&b"xmlns:ram"[..], &super::NS_RAM[..]));
                            bytes.push_attribute((&b"xmlns:udt"[..], &super::NS_UDT[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    ExchangedDocumentTypeSerializerState::Id(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = ExchangedDocumentTypeSerializerState::TypeCode(
                                WithSerializer::serializer(
                                    &self.value.type_code,
                                    Some("ram:TypeCode"),
                                    false,
                                )?,
                            )
                        }
                    },
                    ExchangedDocumentTypeSerializerState::TypeCode(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = ExchangedDocumentTypeSerializerState::IssueDateTime(
                                    WithSerializer::serializer(
                                        &self.value.issue_date_time,
                                        Some("ram:IssueDateTime"),
                                        false,
                                    )?,
                                )
                            }
                        }
                    }
                    ExchangedDocumentTypeSerializerState::IssueDateTime(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = ExchangedDocumentTypeSerializerState::IncludedNote(
                                    IterSerializer::new(
                                        &self.value.included_note[..],
                                        Some("ram:IncludedNote"),
                                        false,
                                    ),
                                )
                            }
                        }
                    }
                    ExchangedDocumentTypeSerializerState::IncludedNote(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = ExchangedDocumentTypeSerializerState::End__,
                        }
                    }
                    ExchangedDocumentTypeSerializerState::End__ => {
                        *self.state = ExchangedDocumentTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    ExchangedDocumentTypeSerializerState::Done__ => return Ok(None),
                    ExchangedDocumentTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for ExchangedDocumentTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ExchangedDocumentTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct SupplyChainTradeTransactionTypeSerializer<'ser> {
        pub(super) value: &'ser super::SupplyChainTradeTransactionType,
        pub(super) state: Box<SupplyChainTradeTransactionTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum SupplyChainTradeTransactionTypeSerializerState<'ser> {
        Init__,
        ApplicableHeaderTradeAgreement(
            <super::HeaderTradeAgreementType as WithSerializer>::Serializer<'ser>,
        ),
        ApplicableHeaderTradeDelivery(
            <super::HeaderTradeDeliveryType as WithSerializer>::Serializer<'ser>,
        ),
        ApplicableHeaderTradeSettlement(
            <super::HeaderTradeSettlementType as WithSerializer>::Serializer<'ser>,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> SupplyChainTradeTransactionTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match & mut * self . state { SupplyChainTradeTransactionTypeSerializerState :: Init__ => { * self . state = SupplyChainTradeTransactionTypeSerializerState :: ApplicableHeaderTradeAgreement (WithSerializer :: serializer (& self . value . applicable_header_trade_agreement , Some ("ram:ApplicableHeaderTradeAgreement") , false) ?) ; let mut bytes = BytesStart :: new (self . name) ; if self . is_root { bytes . push_attribute ((& b"xmlns:rsm" [..] , & super :: NS_RSM [..])) ; bytes . push_attribute ((& b"xmlns:qdt" [..] , & super :: NS_QDT [..])) ; bytes . push_attribute ((& b"xmlns:ram" [..] , & super :: NS_RAM [..])) ; bytes . push_attribute ((& b"xmlns:udt" [..] , & super :: NS_UDT [..])) ; } return Ok (Some (Event :: Start (bytes))) } SupplyChainTradeTransactionTypeSerializerState :: ApplicableHeaderTradeAgreement (x) => match x . next () . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = SupplyChainTradeTransactionTypeSerializerState :: ApplicableHeaderTradeDelivery (WithSerializer :: serializer (& self . value . applicable_header_trade_delivery , Some ("ram:ApplicableHeaderTradeDelivery") , false) ?) , } SupplyChainTradeTransactionTypeSerializerState :: ApplicableHeaderTradeDelivery (x) => match x . next () . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = SupplyChainTradeTransactionTypeSerializerState :: ApplicableHeaderTradeSettlement (WithSerializer :: serializer (& self . value . applicable_header_trade_settlement , Some ("ram:ApplicableHeaderTradeSettlement") , false) ?) , } SupplyChainTradeTransactionTypeSerializerState :: ApplicableHeaderTradeSettlement (x) => match x . next () . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = SupplyChainTradeTransactionTypeSerializerState :: End__ , } SupplyChainTradeTransactionTypeSerializerState :: End__ => { * self . state = SupplyChainTradeTransactionTypeSerializerState :: Done__ ; return Ok (Some (Event :: End (BytesEnd :: new (self . name)))) ; } SupplyChainTradeTransactionTypeSerializerState :: Done__ => return Ok (None) , SupplyChainTradeTransactionTypeSerializerState :: Phantom__ (_) => unreachable ! () , }
            }
        }
    }
    impl<'ser> Iterator for SupplyChainTradeTransactionTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = SupplyChainTradeTransactionTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct DocumentContextParameterTypeSerializer<'ser> {
        pub(super) value: &'ser super::DocumentContextParameterType,
        pub(super) state: Box<DocumentContextParameterTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum DocumentContextParameterTypeSerializerState<'ser> {
        Init__,
        Id(<super::IdType as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> DocumentContextParameterTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    DocumentContextParameterTypeSerializerState::Init__ => {
                        *self.state = DocumentContextParameterTypeSerializerState::Id(
                            WithSerializer::serializer(&self.value.id, Some("ram:ID"), false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:rsm"[..], &super::NS_RSM[..]));
                            bytes.push_attribute((&b"xmlns:qdt"[..], &super::NS_QDT[..]));
                            bytes.push_attribute((&b"xmlns:ram"[..], &super::NS_RAM[..]));
                            bytes.push_attribute((&b"xmlns:udt"[..], &super::NS_UDT[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    DocumentContextParameterTypeSerializerState::Id(x) => match x
                        .next()
                        .transpose()?
                    {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = DocumentContextParameterTypeSerializerState::End__,
                    },
                    DocumentContextParameterTypeSerializerState::End__ => {
                        *self.state = DocumentContextParameterTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    DocumentContextParameterTypeSerializerState::Done__ => return Ok(None),
                    DocumentContextParameterTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for DocumentContextParameterTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = DocumentContextParameterTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct IdTypeSerializer<'ser> {
        pub(super) value: &'ser super::IdType,
        pub(super) state: Box<IdTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum IdTypeSerializerState<'ser> {
        Init__,
        Content__(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> IdTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    IdTypeSerializerState::Init__ => {
                        *self.state = IdTypeSerializerState::Content__(WithSerializer::serializer(
                            &self.value.content,
                            None,
                            false,
                        )?);
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:rsm"[..], &super::NS_RSM[..]));
                            bytes.push_attribute((&b"xmlns:qdt"[..], &super::NS_QDT[..]));
                            bytes.push_attribute((&b"xmlns:ram"[..], &super::NS_RAM[..]));
                            bytes.push_attribute((&b"xmlns:udt"[..], &super::NS_UDT[..]));
                        }
                        write_attrib_opt(&mut bytes, "udt:schemeID", &self.value.scheme_id)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    IdTypeSerializerState::Content__(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = IdTypeSerializerState::End__,
                    },
                    IdTypeSerializerState::End__ => {
                        *self.state = IdTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    IdTypeSerializerState::Done__ => return Ok(None),
                    IdTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for IdTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = IdTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct DocumentCodeTypeSerializer<'ser> {
        pub(super) value: &'ser super::DocumentCodeType,
        pub(super) state: Box<DocumentCodeTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum DocumentCodeTypeSerializerState<'ser> {
        Init__,
        Content__(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> DocumentCodeTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    DocumentCodeTypeSerializerState::Init__ => {
                        *self.state = DocumentCodeTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:rsm"[..], &super::NS_RSM[..]));
                            bytes.push_attribute((&b"xmlns:qdt"[..], &super::NS_QDT[..]));
                            bytes.push_attribute((&b"xmlns:ram"[..], &super::NS_RAM[..]));
                            bytes.push_attribute((&b"xmlns:udt"[..], &super::NS_UDT[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    DocumentCodeTypeSerializerState::Content__(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = DocumentCodeTypeSerializerState::End__,
                    },
                    DocumentCodeTypeSerializerState::End__ => {
                        *self.state = DocumentCodeTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    DocumentCodeTypeSerializerState::Done__ => return Ok(None),
                    DocumentCodeTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for DocumentCodeTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = DocumentCodeTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct DateTimeTypeSerializer<'ser> {
        pub(super) value: &'ser super::DateTimeType,
        pub(super) state: Box<DateTimeTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum DateTimeTypeSerializerState<'ser> {
        Init__,
        Content__(<super::DateTimeTypeContent as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> DateTimeTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    DateTimeTypeSerializerState::Init__ => {
                        *self.state = DateTimeTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:rsm"[..], &super::NS_RSM[..]));
                            bytes.push_attribute((&b"xmlns:qdt"[..], &super::NS_QDT[..]));
                            bytes.push_attribute((&b"xmlns:ram"[..], &super::NS_RAM[..]));
                            bytes.push_attribute((&b"xmlns:udt"[..], &super::NS_UDT[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    DateTimeTypeSerializerState::Content__(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = DateTimeTypeSerializerState::End__,
                    },
                    DateTimeTypeSerializerState::End__ => {
                        *self.state = DateTimeTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    DateTimeTypeSerializerState::Done__ => return Ok(None),
                    DateTimeTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for DateTimeTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = DateTimeTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct DateTimeTypeContentSerializer<'ser> {
        pub(super) value: &'ser super::DateTimeTypeContent,
        pub(super) state: Box<DateTimeTypeContentSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum DateTimeTypeContentSerializerState<'ser> {
        Init__,
        DateTimeString(<super::DateTimeTypeDateTimeStringType as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> DateTimeTypeContentSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    DateTimeTypeContentSerializerState::Init__ => match self.value {
                        super::DateTimeTypeContent::DateTimeString(x) => {
                            *self.state = DateTimeTypeContentSerializerState::DateTimeString(
                                WithSerializer::serializer(x, Some("udt:DateTimeString"), false)?,
                            )
                        }
                    },
                    DateTimeTypeContentSerializerState::DateTimeString(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = DateTimeTypeContentSerializerState::Done__,
                        }
                    }
                    DateTimeTypeContentSerializerState::Done__ => return Ok(None),
                    DateTimeTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for DateTimeTypeContentSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = DateTimeTypeContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct NoteTypeSerializer<'ser> {
        pub(super) value: &'ser super::NoteType,
        pub(super) state: Box<NoteTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum NoteTypeSerializerState<'ser> {
        Init__,
        Content(<super::TextType as WithSerializer>::Serializer<'ser>),
        SubjectCode(IterSerializer<'ser, Option<&'ser super::CodeType>, super::CodeType>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> NoteTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    NoteTypeSerializerState::Init__ => {
                        *self.state = NoteTypeSerializerState::Content(WithSerializer::serializer(
                            &self.value.content,
                            Some("ram:Content"),
                            false,
                        )?);
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:rsm"[..], &super::NS_RSM[..]));
                            bytes.push_attribute((&b"xmlns:qdt"[..], &super::NS_QDT[..]));
                            bytes.push_attribute((&b"xmlns:ram"[..], &super::NS_RAM[..]));
                            bytes.push_attribute((&b"xmlns:udt"[..], &super::NS_UDT[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    NoteTypeSerializerState::Content(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = NoteTypeSerializerState::SubjectCode(IterSerializer::new(
                                self.value.subject_code.as_ref(),
                                Some("ram:SubjectCode"),
                                false,
                            ))
                        }
                    },
                    NoteTypeSerializerState::SubjectCode(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = NoteTypeSerializerState::End__,
                    },
                    NoteTypeSerializerState::End__ => {
                        *self.state = NoteTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    NoteTypeSerializerState::Done__ => return Ok(None),
                    NoteTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for NoteTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = NoteTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct HeaderTradeAgreementTypeSerializer<'ser> {
        pub(super) value: &'ser super::HeaderTradeAgreementType,
        pub(super) state: Box<HeaderTradeAgreementTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum HeaderTradeAgreementTypeSerializerState<'ser> {
        Init__,
        BuyerReference(IterSerializer<'ser, Option<&'ser super::TextType>, super::TextType>),
        SellerTradeParty(<super::TradePartyType as WithSerializer>::Serializer<'ser>),
        BuyerTradeParty(<super::TradePartyType as WithSerializer>::Serializer<'ser>),
        SellerTaxRepresentativeTradeParty(
            IterSerializer<'ser, Option<&'ser super::TradePartyType>, super::TradePartyType>,
        ),
        BuyerOrderReferencedDocument(
            IterSerializer<
                'ser,
                Option<&'ser super::ReferencedDocumentType>,
                super::ReferencedDocumentType,
            >,
        ),
        ContractReferencedDocument(
            IterSerializer<
                'ser,
                Option<&'ser super::ReferencedDocumentType>,
                super::ReferencedDocumentType,
            >,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> HeaderTradeAgreementTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match & mut * self . state { HeaderTradeAgreementTypeSerializerState :: Init__ => { * self . state = HeaderTradeAgreementTypeSerializerState :: BuyerReference (IterSerializer :: new (self . value . buyer_reference . as_ref () , Some ("ram:BuyerReference") , false)) ; let mut bytes = BytesStart :: new (self . name) ; if self . is_root { bytes . push_attribute ((& b"xmlns:rsm" [..] , & super :: NS_RSM [..])) ; bytes . push_attribute ((& b"xmlns:qdt" [..] , & super :: NS_QDT [..])) ; bytes . push_attribute ((& b"xmlns:ram" [..] , & super :: NS_RAM [..])) ; bytes . push_attribute ((& b"xmlns:udt" [..] , & super :: NS_UDT [..])) ; } return Ok (Some (Event :: Start (bytes))) } HeaderTradeAgreementTypeSerializerState :: BuyerReference (x) => match x . next () . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = HeaderTradeAgreementTypeSerializerState :: SellerTradeParty (WithSerializer :: serializer (& self . value . seller_trade_party , Some ("ram:SellerTradeParty") , false) ?) , } HeaderTradeAgreementTypeSerializerState :: SellerTradeParty (x) => match x . next () . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = HeaderTradeAgreementTypeSerializerState :: BuyerTradeParty (WithSerializer :: serializer (& self . value . buyer_trade_party , Some ("ram:BuyerTradeParty") , false) ?) , } HeaderTradeAgreementTypeSerializerState :: BuyerTradeParty (x) => match x . next () . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = HeaderTradeAgreementTypeSerializerState :: SellerTaxRepresentativeTradeParty (IterSerializer :: new (self . value . seller_tax_representative_trade_party . as_ref () , Some ("ram:SellerTaxRepresentativeTradeParty") , false)) , } HeaderTradeAgreementTypeSerializerState :: SellerTaxRepresentativeTradeParty (x) => match x . next () . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = HeaderTradeAgreementTypeSerializerState :: BuyerOrderReferencedDocument (IterSerializer :: new (self . value . buyer_order_referenced_document . as_ref () , Some ("ram:BuyerOrderReferencedDocument") , false)) , } HeaderTradeAgreementTypeSerializerState :: BuyerOrderReferencedDocument (x) => match x . next () . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = HeaderTradeAgreementTypeSerializerState :: ContractReferencedDocument (IterSerializer :: new (self . value . contract_referenced_document . as_ref () , Some ("ram:ContractReferencedDocument") , false)) , } HeaderTradeAgreementTypeSerializerState :: ContractReferencedDocument (x) => match x . next () . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = HeaderTradeAgreementTypeSerializerState :: End__ , } HeaderTradeAgreementTypeSerializerState :: End__ => { * self . state = HeaderTradeAgreementTypeSerializerState :: Done__ ; return Ok (Some (Event :: End (BytesEnd :: new (self . name)))) ; } HeaderTradeAgreementTypeSerializerState :: Done__ => return Ok (None) , HeaderTradeAgreementTypeSerializerState :: Phantom__ (_) => unreachable ! () , }
            }
        }
    }
    impl<'ser> Iterator for HeaderTradeAgreementTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = HeaderTradeAgreementTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct HeaderTradeDeliveryTypeSerializer<'ser> {
        pub(super) value: &'ser super::HeaderTradeDeliveryType,
        pub(super) state: Box<HeaderTradeDeliveryTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum HeaderTradeDeliveryTypeSerializerState<'ser> {
        Init__,
        ShipToTradeParty(
            IterSerializer<'ser, Option<&'ser super::TradePartyType>, super::TradePartyType>,
        ),
        ActualDeliverySupplyChainEvent(
            IterSerializer<
                'ser,
                Option<&'ser super::SupplyChainEventType>,
                super::SupplyChainEventType,
            >,
        ),
        DespatchAdviceReferencedDocument(
            IterSerializer<
                'ser,
                Option<&'ser super::ReferencedDocumentType>,
                super::ReferencedDocumentType,
            >,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> HeaderTradeDeliveryTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match & mut * self . state { HeaderTradeDeliveryTypeSerializerState :: Init__ => { * self . state = HeaderTradeDeliveryTypeSerializerState :: ShipToTradeParty (IterSerializer :: new (self . value . ship_to_trade_party . as_ref () , Some ("ram:ShipToTradeParty") , false)) ; let mut bytes = BytesStart :: new (self . name) ; if self . is_root { bytes . push_attribute ((& b"xmlns:rsm" [..] , & super :: NS_RSM [..])) ; bytes . push_attribute ((& b"xmlns:qdt" [..] , & super :: NS_QDT [..])) ; bytes . push_attribute ((& b"xmlns:ram" [..] , & super :: NS_RAM [..])) ; bytes . push_attribute ((& b"xmlns:udt" [..] , & super :: NS_UDT [..])) ; } return Ok (Some (Event :: Start (bytes))) } HeaderTradeDeliveryTypeSerializerState :: ShipToTradeParty (x) => match x . next () . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = HeaderTradeDeliveryTypeSerializerState :: ActualDeliverySupplyChainEvent (IterSerializer :: new (self . value . actual_delivery_supply_chain_event . as_ref () , Some ("ram:ActualDeliverySupplyChainEvent") , false)) , } HeaderTradeDeliveryTypeSerializerState :: ActualDeliverySupplyChainEvent (x) => match x . next () . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = HeaderTradeDeliveryTypeSerializerState :: DespatchAdviceReferencedDocument (IterSerializer :: new (self . value . despatch_advice_referenced_document . as_ref () , Some ("ram:DespatchAdviceReferencedDocument") , false)) , } HeaderTradeDeliveryTypeSerializerState :: DespatchAdviceReferencedDocument (x) => match x . next () . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = HeaderTradeDeliveryTypeSerializerState :: End__ , } HeaderTradeDeliveryTypeSerializerState :: End__ => { * self . state = HeaderTradeDeliveryTypeSerializerState :: Done__ ; return Ok (Some (Event :: End (BytesEnd :: new (self . name)))) ; } HeaderTradeDeliveryTypeSerializerState :: Done__ => return Ok (None) , HeaderTradeDeliveryTypeSerializerState :: Phantom__ (_) => unreachable ! () , }
            }
        }
    }
    impl<'ser> Iterator for HeaderTradeDeliveryTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = HeaderTradeDeliveryTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct HeaderTradeSettlementTypeSerializer<'ser> {
        pub(super) value: &'ser super::HeaderTradeSettlementType,
        pub(super) state: Box<HeaderTradeSettlementTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum HeaderTradeSettlementTypeSerializerState<'ser> {
        Init__,
        CreditorReferenceId(IterSerializer<'ser, Option<&'ser super::IdType>, super::IdType>),
        PaymentReference(IterSerializer<'ser, Option<&'ser super::TextType>, super::TextType>),
        TaxCurrencyCode(
            IterSerializer<'ser, Option<&'ser super::CurrencyCodeType>, super::CurrencyCodeType>,
        ),
        InvoiceCurrencyCode(<super::CurrencyCodeType as WithSerializer>::Serializer<'ser>),
        PayeeTradeParty(
            IterSerializer<'ser, Option<&'ser super::TradePartyType>, super::TradePartyType>,
        ),
        SpecifiedTradeSettlementPaymentMeans(
            IterSerializer<
                'ser,
                &'ser [super::TradeSettlementPaymentMeansType],
                super::TradeSettlementPaymentMeansType,
            >,
        ),
        ApplicableTradeTax(IterSerializer<'ser, &'ser [super::TradeTaxType], super::TradeTaxType>),
        BillingSpecifiedPeriod(
            IterSerializer<
                'ser,
                Option<&'ser super::SpecifiedPeriodType>,
                super::SpecifiedPeriodType,
            >,
        ),
        SpecifiedTradeAllowanceCharge(
            IterSerializer<
                'ser,
                &'ser [super::TradeAllowanceChargeType],
                super::TradeAllowanceChargeType,
            >,
        ),
        SpecifiedTradePaymentTerms(
            IterSerializer<
                'ser,
                Option<&'ser super::TradePaymentTermsType>,
                super::TradePaymentTermsType,
            >,
        ),
        SpecifiedTradeSettlementHeaderMonetarySummation(
            <super::TradeSettlementHeaderMonetarySummationType as WithSerializer>::Serializer<'ser>,
        ),
        InvoiceReferencedDocument(
            IterSerializer<
                'ser,
                &'ser [super::ReferencedDocumentType],
                super::ReferencedDocumentType,
            >,
        ),
        ReceivableSpecifiedTradeAccountingAccount(
            IterSerializer<
                'ser,
                Option<&'ser super::TradeAccountingAccountType>,
                super::TradeAccountingAccountType,
            >,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> HeaderTradeSettlementTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match & mut * self . state { HeaderTradeSettlementTypeSerializerState :: Init__ => { * self . state = HeaderTradeSettlementTypeSerializerState :: CreditorReferenceId (IterSerializer :: new (self . value . creditor_reference_id . as_ref () , Some ("ram:CreditorReferenceID") , false)) ; let mut bytes = BytesStart :: new (self . name) ; if self . is_root { bytes . push_attribute ((& b"xmlns:rsm" [..] , & super :: NS_RSM [..])) ; bytes . push_attribute ((& b"xmlns:qdt" [..] , & super :: NS_QDT [..])) ; bytes . push_attribute ((& b"xmlns:ram" [..] , & super :: NS_RAM [..])) ; bytes . push_attribute ((& b"xmlns:udt" [..] , & super :: NS_UDT [..])) ; } return Ok (Some (Event :: Start (bytes))) } HeaderTradeSettlementTypeSerializerState :: CreditorReferenceId (x) => match x . next () . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = HeaderTradeSettlementTypeSerializerState :: PaymentReference (IterSerializer :: new (self . value . payment_reference . as_ref () , Some ("ram:PaymentReference") , false)) , } HeaderTradeSettlementTypeSerializerState :: PaymentReference (x) => match x . next () . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = HeaderTradeSettlementTypeSerializerState :: TaxCurrencyCode (IterSerializer :: new (self . value . tax_currency_code . as_ref () , Some ("ram:TaxCurrencyCode") , false)) , } HeaderTradeSettlementTypeSerializerState :: TaxCurrencyCode (x) => match x . next () . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = HeaderTradeSettlementTypeSerializerState :: InvoiceCurrencyCode (WithSerializer :: serializer (& self . value . invoice_currency_code , Some ("ram:InvoiceCurrencyCode") , false) ?) , } HeaderTradeSettlementTypeSerializerState :: InvoiceCurrencyCode (x) => match x . next () . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = HeaderTradeSettlementTypeSerializerState :: PayeeTradeParty (IterSerializer :: new (self . value . payee_trade_party . as_ref () , Some ("ram:PayeeTradeParty") , false)) , } HeaderTradeSettlementTypeSerializerState :: PayeeTradeParty (x) => match x . next () . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = HeaderTradeSettlementTypeSerializerState :: SpecifiedTradeSettlementPaymentMeans (IterSerializer :: new (& self . value . specified_trade_settlement_payment_means [..] , Some ("ram:SpecifiedTradeSettlementPaymentMeans") , false)) , } HeaderTradeSettlementTypeSerializerState :: SpecifiedTradeSettlementPaymentMeans (x) => match x . next () . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = HeaderTradeSettlementTypeSerializerState :: ApplicableTradeTax (IterSerializer :: new (& self . value . applicable_trade_tax [..] , Some ("ram:ApplicableTradeTax") , false)) , } HeaderTradeSettlementTypeSerializerState :: ApplicableTradeTax (x) => match x . next () . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = HeaderTradeSettlementTypeSerializerState :: BillingSpecifiedPeriod (IterSerializer :: new (self . value . billing_specified_period . as_ref () , Some ("ram:BillingSpecifiedPeriod") , false)) , } HeaderTradeSettlementTypeSerializerState :: BillingSpecifiedPeriod (x) => match x . next () . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = HeaderTradeSettlementTypeSerializerState :: SpecifiedTradeAllowanceCharge (IterSerializer :: new (& self . value . specified_trade_allowance_charge [..] , Some ("ram:SpecifiedTradeAllowanceCharge") , false)) , } HeaderTradeSettlementTypeSerializerState :: SpecifiedTradeAllowanceCharge (x) => match x . next () . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = HeaderTradeSettlementTypeSerializerState :: SpecifiedTradePaymentTerms (IterSerializer :: new (self . value . specified_trade_payment_terms . as_ref () , Some ("ram:SpecifiedTradePaymentTerms") , false)) , } HeaderTradeSettlementTypeSerializerState :: SpecifiedTradePaymentTerms (x) => match x . next () . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = HeaderTradeSettlementTypeSerializerState :: SpecifiedTradeSettlementHeaderMonetarySummation (WithSerializer :: serializer (& self . value . specified_trade_settlement_header_monetary_summation , Some ("ram:SpecifiedTradeSettlementHeaderMonetarySummation") , false) ?) , } HeaderTradeSettlementTypeSerializerState :: SpecifiedTradeSettlementHeaderMonetarySummation (x) => match x . next () . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = HeaderTradeSettlementTypeSerializerState :: InvoiceReferencedDocument (IterSerializer :: new (& self . value . invoice_referenced_document [..] , Some ("ram:InvoiceReferencedDocument") , false)) , } HeaderTradeSettlementTypeSerializerState :: InvoiceReferencedDocument (x) => match x . next () . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = HeaderTradeSettlementTypeSerializerState :: ReceivableSpecifiedTradeAccountingAccount (IterSerializer :: new (self . value . receivable_specified_trade_accounting_account . as_ref () , Some ("ram:ReceivableSpecifiedTradeAccountingAccount") , false)) , } HeaderTradeSettlementTypeSerializerState :: ReceivableSpecifiedTradeAccountingAccount (x) => match x . next () . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = HeaderTradeSettlementTypeSerializerState :: End__ , } HeaderTradeSettlementTypeSerializerState :: End__ => { * self . state = HeaderTradeSettlementTypeSerializerState :: Done__ ; return Ok (Some (Event :: End (BytesEnd :: new (self . name)))) ; } HeaderTradeSettlementTypeSerializerState :: Done__ => return Ok (None) , HeaderTradeSettlementTypeSerializerState :: Phantom__ (_) => unreachable ! () , }
            }
        }
    }
    impl<'ser> Iterator for HeaderTradeSettlementTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = HeaderTradeSettlementTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct DateTimeTypeDateTimeStringTypeSerializer<'ser> {
        pub(super) value: &'ser super::DateTimeTypeDateTimeStringType,
        pub(super) state: Box<DateTimeTypeDateTimeStringTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum DateTimeTypeDateTimeStringTypeSerializerState<'ser> {
        Init__,
        Content__(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> DateTimeTypeDateTimeStringTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    DateTimeTypeDateTimeStringTypeSerializerState::Init__ => {
                        *self.state = DateTimeTypeDateTimeStringTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:rsm"[..], &super::NS_RSM[..]));
                            bytes.push_attribute((&b"xmlns:qdt"[..], &super::NS_QDT[..]));
                            bytes.push_attribute((&b"xmlns:ram"[..], &super::NS_RAM[..]));
                            bytes.push_attribute((&b"xmlns:udt"[..], &super::NS_UDT[..]));
                        }
                        write_attrib(&mut bytes, "udt:format", &self.value.format)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    DateTimeTypeDateTimeStringTypeSerializerState::Content__(x) => match x
                        .next()
                        .transpose()?
                    {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = DateTimeTypeDateTimeStringTypeSerializerState::End__,
                    },
                    DateTimeTypeDateTimeStringTypeSerializerState::End__ => {
                        *self.state = DateTimeTypeDateTimeStringTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    DateTimeTypeDateTimeStringTypeSerializerState::Done__ => return Ok(None),
                    DateTimeTypeDateTimeStringTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for DateTimeTypeDateTimeStringTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = DateTimeTypeDateTimeStringTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct TextTypeSerializer<'ser> {
        pub(super) value: &'ser super::TextType,
        pub(super) state: Box<TextTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum TextTypeSerializerState<'ser> {
        Init__,
        Content__(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> TextTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    TextTypeSerializerState::Init__ => {
                        *self.state = TextTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:rsm"[..], &super::NS_RSM[..]));
                            bytes.push_attribute((&b"xmlns:qdt"[..], &super::NS_QDT[..]));
                            bytes.push_attribute((&b"xmlns:ram"[..], &super::NS_RAM[..]));
                            bytes.push_attribute((&b"xmlns:udt"[..], &super::NS_UDT[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    TextTypeSerializerState::Content__(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = TextTypeSerializerState::End__,
                    },
                    TextTypeSerializerState::End__ => {
                        *self.state = TextTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    TextTypeSerializerState::Done__ => return Ok(None),
                    TextTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for TextTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = TextTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct CodeTypeSerializer<'ser> {
        pub(super) value: &'ser super::CodeType,
        pub(super) state: Box<CodeTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum CodeTypeSerializerState<'ser> {
        Init__,
        Content__(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> CodeTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    CodeTypeSerializerState::Init__ => {
                        *self.state = CodeTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:rsm"[..], &super::NS_RSM[..]));
                            bytes.push_attribute((&b"xmlns:qdt"[..], &super::NS_QDT[..]));
                            bytes.push_attribute((&b"xmlns:ram"[..], &super::NS_RAM[..]));
                            bytes.push_attribute((&b"xmlns:udt"[..], &super::NS_UDT[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    CodeTypeSerializerState::Content__(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = CodeTypeSerializerState::End__,
                    },
                    CodeTypeSerializerState::End__ => {
                        *self.state = CodeTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    CodeTypeSerializerState::Done__ => return Ok(None),
                    CodeTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for CodeTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = CodeTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct TradePartyTypeSerializer<'ser> {
        pub(super) value: &'ser super::TradePartyType,
        pub(super) state: Box<TradePartyTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum TradePartyTypeSerializerState<'ser> {
        Init__,
        Id(IterSerializer<'ser, &'ser [super::IdType], super::IdType>),
        GlobalId(IterSerializer<'ser, &'ser [super::IdType], super::IdType>),
        Name(IterSerializer<'ser, Option<&'ser super::TextType>, super::TextType>),
        SpecifiedLegalOrganization(
            IterSerializer<
                'ser,
                Option<&'ser super::LegalOrganizationType>,
                super::LegalOrganizationType,
            >,
        ),
        PostalTradeAddress(
            IterSerializer<'ser, Option<&'ser super::TradeAddressType>, super::TradeAddressType>,
        ),
        UriUniversalCommunication(
            IterSerializer<
                'ser,
                Option<&'ser super::UniversalCommunicationType>,
                super::UniversalCommunicationType,
            >,
        ),
        SpecifiedTaxRegistration(
            IterSerializer<'ser, &'ser [super::TaxRegistrationType], super::TaxRegistrationType>,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> TradePartyTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    TradePartyTypeSerializerState::Init__ => {
                        *self.state = TradePartyTypeSerializerState::Id(IterSerializer::new(
                            &self.value.id[..],
                            Some("ram:ID"),
                            false,
                        ));
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:rsm"[..], &super::NS_RSM[..]));
                            bytes.push_attribute((&b"xmlns:qdt"[..], &super::NS_QDT[..]));
                            bytes.push_attribute((&b"xmlns:ram"[..], &super::NS_RAM[..]));
                            bytes.push_attribute((&b"xmlns:udt"[..], &super::NS_UDT[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    TradePartyTypeSerializerState::Id(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state =
                                TradePartyTypeSerializerState::GlobalId(IterSerializer::new(
                                    &self.value.global_id[..],
                                    Some("ram:GlobalID"),
                                    false,
                                ))
                        }
                    },
                    TradePartyTypeSerializerState::GlobalId(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = TradePartyTypeSerializerState::Name(IterSerializer::new(
                                self.value.name.as_ref(),
                                Some("ram:Name"),
                                false,
                            ))
                        }
                    },
                    TradePartyTypeSerializerState::Name(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = TradePartyTypeSerializerState::SpecifiedLegalOrganization(
                                IterSerializer::new(
                                    self.value.specified_legal_organization.as_ref(),
                                    Some("ram:SpecifiedLegalOrganization"),
                                    false,
                                ),
                            )
                        }
                    },
                    TradePartyTypeSerializerState::SpecifiedLegalOrganization(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = TradePartyTypeSerializerState::PostalTradeAddress(
                                    IterSerializer::new(
                                        self.value.postal_trade_address.as_ref(),
                                        Some("ram:PostalTradeAddress"),
                                        false,
                                    ),
                                )
                            }
                        }
                    }
                    TradePartyTypeSerializerState::PostalTradeAddress(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state =
                                    TradePartyTypeSerializerState::UriUniversalCommunication(
                                        IterSerializer::new(
                                            self.value.uri_universal_communication.as_ref(),
                                            Some("ram:URIUniversalCommunication"),
                                            false,
                                        ),
                                    )
                            }
                        }
                    }
                    TradePartyTypeSerializerState::UriUniversalCommunication(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state =
                                    TradePartyTypeSerializerState::SpecifiedTaxRegistration(
                                        IterSerializer::new(
                                            &self.value.specified_tax_registration[..],
                                            Some("ram:SpecifiedTaxRegistration"),
                                            false,
                                        ),
                                    )
                            }
                        }
                    }
                    TradePartyTypeSerializerState::SpecifiedTaxRegistration(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = TradePartyTypeSerializerState::End__,
                        }
                    }
                    TradePartyTypeSerializerState::End__ => {
                        *self.state = TradePartyTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    TradePartyTypeSerializerState::Done__ => return Ok(None),
                    TradePartyTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for TradePartyTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = TradePartyTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct ReferencedDocumentTypeSerializer<'ser> {
        pub(super) value: &'ser super::ReferencedDocumentType,
        pub(super) state: Box<ReferencedDocumentTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum ReferencedDocumentTypeSerializerState<'ser> {
        Init__,
        IssuerAssignedId(<super::IdType as WithSerializer>::Serializer<'ser>),
        FormattedIssueDateTime(
            IterSerializer<
                'ser,
                Option<&'ser super::FormattedDateTimeType>,
                super::FormattedDateTimeType,
            >,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ReferencedDocumentTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    ReferencedDocumentTypeSerializerState::Init__ => {
                        *self.state = ReferencedDocumentTypeSerializerState::IssuerAssignedId(
                            WithSerializer::serializer(
                                &self.value.issuer_assigned_id,
                                Some("ram:IssuerAssignedID"),
                                false,
                            )?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:rsm"[..], &super::NS_RSM[..]));
                            bytes.push_attribute((&b"xmlns:qdt"[..], &super::NS_QDT[..]));
                            bytes.push_attribute((&b"xmlns:ram"[..], &super::NS_RAM[..]));
                            bytes.push_attribute((&b"xmlns:udt"[..], &super::NS_UDT[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    ReferencedDocumentTypeSerializerState::IssuerAssignedId(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state =
                                    ReferencedDocumentTypeSerializerState::FormattedIssueDateTime(
                                        IterSerializer::new(
                                            self.value.formatted_issue_date_time.as_ref(),
                                            Some("ram:FormattedIssueDateTime"),
                                            false,
                                        ),
                                    )
                            }
                        }
                    }
                    ReferencedDocumentTypeSerializerState::FormattedIssueDateTime(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = ReferencedDocumentTypeSerializerState::End__,
                        }
                    }
                    ReferencedDocumentTypeSerializerState::End__ => {
                        *self.state = ReferencedDocumentTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    ReferencedDocumentTypeSerializerState::Done__ => return Ok(None),
                    ReferencedDocumentTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for ReferencedDocumentTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ReferencedDocumentTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct SupplyChainEventTypeSerializer<'ser> {
        pub(super) value: &'ser super::SupplyChainEventType,
        pub(super) state: Box<SupplyChainEventTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum SupplyChainEventTypeSerializerState<'ser> {
        Init__,
        OccurrenceDateTime(<super::DateTimeType as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> SupplyChainEventTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    SupplyChainEventTypeSerializerState::Init__ => {
                        *self.state = SupplyChainEventTypeSerializerState::OccurrenceDateTime(
                            WithSerializer::serializer(
                                &self.value.occurrence_date_time,
                                Some("ram:OccurrenceDateTime"),
                                false,
                            )?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:rsm"[..], &super::NS_RSM[..]));
                            bytes.push_attribute((&b"xmlns:qdt"[..], &super::NS_QDT[..]));
                            bytes.push_attribute((&b"xmlns:ram"[..], &super::NS_RAM[..]));
                            bytes.push_attribute((&b"xmlns:udt"[..], &super::NS_UDT[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    SupplyChainEventTypeSerializerState::OccurrenceDateTime(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = SupplyChainEventTypeSerializerState::End__,
                        }
                    }
                    SupplyChainEventTypeSerializerState::End__ => {
                        *self.state = SupplyChainEventTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    SupplyChainEventTypeSerializerState::Done__ => return Ok(None),
                    SupplyChainEventTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for SupplyChainEventTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = SupplyChainEventTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct CurrencyCodeTypeSerializer<'ser> {
        pub(super) value: &'ser super::CurrencyCodeType,
        pub(super) state: Box<CurrencyCodeTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum CurrencyCodeTypeSerializerState<'ser> {
        Init__,
        Content__(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> CurrencyCodeTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    CurrencyCodeTypeSerializerState::Init__ => {
                        *self.state = CurrencyCodeTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:rsm"[..], &super::NS_RSM[..]));
                            bytes.push_attribute((&b"xmlns:qdt"[..], &super::NS_QDT[..]));
                            bytes.push_attribute((&b"xmlns:ram"[..], &super::NS_RAM[..]));
                            bytes.push_attribute((&b"xmlns:udt"[..], &super::NS_UDT[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    CurrencyCodeTypeSerializerState::Content__(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = CurrencyCodeTypeSerializerState::End__,
                    },
                    CurrencyCodeTypeSerializerState::End__ => {
                        *self.state = CurrencyCodeTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    CurrencyCodeTypeSerializerState::Done__ => return Ok(None),
                    CurrencyCodeTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for CurrencyCodeTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = CurrencyCodeTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct TradeSettlementPaymentMeansTypeSerializer<'ser> {
        pub(super) value: &'ser super::TradeSettlementPaymentMeansType,
        pub(super) state: Box<TradeSettlementPaymentMeansTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum TradeSettlementPaymentMeansTypeSerializerState<'ser> {
        Init__,
        TypeCode(<super::PaymentMeansCodeType as WithSerializer>::Serializer<'ser>),
        PayerPartyDebtorFinancialAccount(
            IterSerializer<
                'ser,
                Option<&'ser super::DebtorFinancialAccountType>,
                super::DebtorFinancialAccountType,
            >,
        ),
        PayeePartyCreditorFinancialAccount(
            IterSerializer<
                'ser,
                Option<&'ser super::CreditorFinancialAccountType>,
                super::CreditorFinancialAccountType,
            >,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> TradeSettlementPaymentMeansTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match & mut * self . state { TradeSettlementPaymentMeansTypeSerializerState :: Init__ => { * self . state = TradeSettlementPaymentMeansTypeSerializerState :: TypeCode (WithSerializer :: serializer (& self . value . type_code , Some ("ram:TypeCode") , false) ?) ; let mut bytes = BytesStart :: new (self . name) ; if self . is_root { bytes . push_attribute ((& b"xmlns:rsm" [..] , & super :: NS_RSM [..])) ; bytes . push_attribute ((& b"xmlns:qdt" [..] , & super :: NS_QDT [..])) ; bytes . push_attribute ((& b"xmlns:ram" [..] , & super :: NS_RAM [..])) ; bytes . push_attribute ((& b"xmlns:udt" [..] , & super :: NS_UDT [..])) ; } return Ok (Some (Event :: Start (bytes))) } TradeSettlementPaymentMeansTypeSerializerState :: TypeCode (x) => match x . next () . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = TradeSettlementPaymentMeansTypeSerializerState :: PayerPartyDebtorFinancialAccount (IterSerializer :: new (self . value . payer_party_debtor_financial_account . as_ref () , Some ("ram:PayerPartyDebtorFinancialAccount") , false)) , } TradeSettlementPaymentMeansTypeSerializerState :: PayerPartyDebtorFinancialAccount (x) => match x . next () . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = TradeSettlementPaymentMeansTypeSerializerState :: PayeePartyCreditorFinancialAccount (IterSerializer :: new (self . value . payee_party_creditor_financial_account . as_ref () , Some ("ram:PayeePartyCreditorFinancialAccount") , false)) , } TradeSettlementPaymentMeansTypeSerializerState :: PayeePartyCreditorFinancialAccount (x) => match x . next () . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = TradeSettlementPaymentMeansTypeSerializerState :: End__ , } TradeSettlementPaymentMeansTypeSerializerState :: End__ => { * self . state = TradeSettlementPaymentMeansTypeSerializerState :: Done__ ; return Ok (Some (Event :: End (BytesEnd :: new (self . name)))) ; } TradeSettlementPaymentMeansTypeSerializerState :: Done__ => return Ok (None) , TradeSettlementPaymentMeansTypeSerializerState :: Phantom__ (_) => unreachable ! () , }
            }
        }
    }
    impl<'ser> Iterator for TradeSettlementPaymentMeansTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = TradeSettlementPaymentMeansTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct TradeTaxTypeSerializer<'ser> {
        pub(super) value: &'ser super::TradeTaxType,
        pub(super) state: Box<TradeTaxTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum TradeTaxTypeSerializerState<'ser> {
        Init__,
        CalculatedAmount(IterSerializer<'ser, Option<&'ser super::AmountType>, super::AmountType>),
        TypeCode(<super::TaxTypeCodeType as WithSerializer>::Serializer<'ser>),
        ExemptionReason(IterSerializer<'ser, Option<&'ser super::TextType>, super::TextType>),
        BasisAmount(IterSerializer<'ser, Option<&'ser super::AmountType>, super::AmountType>),
        CategoryCode(<super::TaxCategoryCodeType as WithSerializer>::Serializer<'ser>),
        ExemptionReasonCode(IterSerializer<'ser, Option<&'ser super::CodeType>, super::CodeType>),
        DueDateTypeCode(
            IterSerializer<
                'ser,
                Option<&'ser super::TimeReferenceCodeType>,
                super::TimeReferenceCodeType,
            >,
        ),
        RateApplicablePercent(
            IterSerializer<'ser, Option<&'ser super::PercentType>, super::PercentType>,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> TradeTaxTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    TradeTaxTypeSerializerState::Init__ => {
                        *self.state =
                            TradeTaxTypeSerializerState::CalculatedAmount(IterSerializer::new(
                                self.value.calculated_amount.as_ref(),
                                Some("ram:CalculatedAmount"),
                                false,
                            ));
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:rsm"[..], &super::NS_RSM[..]));
                            bytes.push_attribute((&b"xmlns:qdt"[..], &super::NS_QDT[..]));
                            bytes.push_attribute((&b"xmlns:ram"[..], &super::NS_RAM[..]));
                            bytes.push_attribute((&b"xmlns:udt"[..], &super::NS_UDT[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    TradeTaxTypeSerializerState::CalculatedAmount(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = TradeTaxTypeSerializerState::TypeCode(
                                    WithSerializer::serializer(
                                        &self.value.type_code,
                                        Some("ram:TypeCode"),
                                        false,
                                    )?,
                                )
                            }
                        }
                    }
                    TradeTaxTypeSerializerState::TypeCode(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state =
                                TradeTaxTypeSerializerState::ExemptionReason(IterSerializer::new(
                                    self.value.exemption_reason.as_ref(),
                                    Some("ram:ExemptionReason"),
                                    false,
                                ))
                        }
                    },
                    TradeTaxTypeSerializerState::ExemptionReason(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state =
                                    TradeTaxTypeSerializerState::BasisAmount(IterSerializer::new(
                                        self.value.basis_amount.as_ref(),
                                        Some("ram:BasisAmount"),
                                        false,
                                    ))
                            }
                        }
                    }
                    TradeTaxTypeSerializerState::BasisAmount(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = TradeTaxTypeSerializerState::CategoryCode(
                                WithSerializer::serializer(
                                    &self.value.category_code,
                                    Some("ram:CategoryCode"),
                                    false,
                                )?,
                            )
                        }
                    },
                    TradeTaxTypeSerializerState::CategoryCode(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = TradeTaxTypeSerializerState::ExemptionReasonCode(
                                IterSerializer::new(
                                    self.value.exemption_reason_code.as_ref(),
                                    Some("ram:ExemptionReasonCode"),
                                    false,
                                ),
                            )
                        }
                    },
                    TradeTaxTypeSerializerState::ExemptionReasonCode(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = TradeTaxTypeSerializerState::DueDateTypeCode(
                                    IterSerializer::new(
                                        self.value.due_date_type_code.as_ref(),
                                        Some("ram:DueDateTypeCode"),
                                        false,
                                    ),
                                )
                            }
                        }
                    }
                    TradeTaxTypeSerializerState::DueDateTypeCode(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = TradeTaxTypeSerializerState::RateApplicablePercent(
                                    IterSerializer::new(
                                        self.value.rate_applicable_percent.as_ref(),
                                        Some("ram:RateApplicablePercent"),
                                        false,
                                    ),
                                )
                            }
                        }
                    }
                    TradeTaxTypeSerializerState::RateApplicablePercent(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = TradeTaxTypeSerializerState::End__,
                        }
                    }
                    TradeTaxTypeSerializerState::End__ => {
                        *self.state = TradeTaxTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    TradeTaxTypeSerializerState::Done__ => return Ok(None),
                    TradeTaxTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for TradeTaxTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = TradeTaxTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct SpecifiedPeriodTypeSerializer<'ser> {
        pub(super) value: &'ser super::SpecifiedPeriodType,
        pub(super) state: Box<SpecifiedPeriodTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum SpecifiedPeriodTypeSerializerState<'ser> {
        Init__,
        StartDateTime(IterSerializer<'ser, Option<&'ser super::DateTimeType>, super::DateTimeType>),
        EndDateTime(IterSerializer<'ser, Option<&'ser super::DateTimeType>, super::DateTimeType>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> SpecifiedPeriodTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    SpecifiedPeriodTypeSerializerState::Init__ => {
                        *self.state =
                            SpecifiedPeriodTypeSerializerState::StartDateTime(IterSerializer::new(
                                self.value.start_date_time.as_ref(),
                                Some("ram:StartDateTime"),
                                false,
                            ));
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:rsm"[..], &super::NS_RSM[..]));
                            bytes.push_attribute((&b"xmlns:qdt"[..], &super::NS_QDT[..]));
                            bytes.push_attribute((&b"xmlns:ram"[..], &super::NS_RAM[..]));
                            bytes.push_attribute((&b"xmlns:udt"[..], &super::NS_UDT[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    SpecifiedPeriodTypeSerializerState::StartDateTime(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = SpecifiedPeriodTypeSerializerState::EndDateTime(
                                    IterSerializer::new(
                                        self.value.end_date_time.as_ref(),
                                        Some("ram:EndDateTime"),
                                        false,
                                    ),
                                )
                            }
                        }
                    }
                    SpecifiedPeriodTypeSerializerState::EndDateTime(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = SpecifiedPeriodTypeSerializerState::End__,
                        }
                    }
                    SpecifiedPeriodTypeSerializerState::End__ => {
                        *self.state = SpecifiedPeriodTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    SpecifiedPeriodTypeSerializerState::Done__ => return Ok(None),
                    SpecifiedPeriodTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for SpecifiedPeriodTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = SpecifiedPeriodTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct TradeAllowanceChargeTypeSerializer<'ser> {
        pub(super) value: &'ser super::TradeAllowanceChargeType,
        pub(super) state: Box<TradeAllowanceChargeTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum TradeAllowanceChargeTypeSerializerState<'ser> {
        Init__,
        ChargeIndicator(<super::IndicatorType as WithSerializer>::Serializer<'ser>),
        CalculationPercent(
            IterSerializer<'ser, Option<&'ser super::PercentType>, super::PercentType>,
        ),
        BasisAmount(IterSerializer<'ser, Option<&'ser super::AmountType>, super::AmountType>),
        ActualAmount(<super::AmountType as WithSerializer>::Serializer<'ser>),
        ReasonCode(
            IterSerializer<
                'ser,
                Option<&'ser super::AllowanceChargeReasonCodeType>,
                super::AllowanceChargeReasonCodeType,
            >,
        ),
        Reason(IterSerializer<'ser, Option<&'ser super::TextType>, super::TextType>),
        CategoryTradeTax(<super::TradeTaxType as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> TradeAllowanceChargeTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    TradeAllowanceChargeTypeSerializerState::Init__ => {
                        *self.state = TradeAllowanceChargeTypeSerializerState::ChargeIndicator(
                            WithSerializer::serializer(
                                &self.value.charge_indicator,
                                Some("ram:ChargeIndicator"),
                                false,
                            )?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:rsm"[..], &super::NS_RSM[..]));
                            bytes.push_attribute((&b"xmlns:qdt"[..], &super::NS_QDT[..]));
                            bytes.push_attribute((&b"xmlns:ram"[..], &super::NS_RAM[..]));
                            bytes.push_attribute((&b"xmlns:udt"[..], &super::NS_UDT[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    TradeAllowanceChargeTypeSerializerState::ChargeIndicator(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state =
                                    TradeAllowanceChargeTypeSerializerState::CalculationPercent(
                                        IterSerializer::new(
                                            self.value.calculation_percent.as_ref(),
                                            Some("ram:CalculationPercent"),
                                            false,
                                        ),
                                    )
                            }
                        }
                    }
                    TradeAllowanceChargeTypeSerializerState::CalculationPercent(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = TradeAllowanceChargeTypeSerializerState::BasisAmount(
                                    IterSerializer::new(
                                        self.value.basis_amount.as_ref(),
                                        Some("ram:BasisAmount"),
                                        false,
                                    ),
                                )
                            }
                        }
                    }
                    TradeAllowanceChargeTypeSerializerState::BasisAmount(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = TradeAllowanceChargeTypeSerializerState::ActualAmount(
                                    WithSerializer::serializer(
                                        &self.value.actual_amount,
                                        Some("ram:ActualAmount"),
                                        false,
                                    )?,
                                )
                            }
                        }
                    }
                    TradeAllowanceChargeTypeSerializerState::ActualAmount(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = TradeAllowanceChargeTypeSerializerState::ReasonCode(
                                    IterSerializer::new(
                                        self.value.reason_code.as_ref(),
                                        Some("ram:ReasonCode"),
                                        false,
                                    ),
                                )
                            }
                        }
                    }
                    TradeAllowanceChargeTypeSerializerState::ReasonCode(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = TradeAllowanceChargeTypeSerializerState::Reason(
                                    IterSerializer::new(
                                        self.value.reason.as_ref(),
                                        Some("ram:Reason"),
                                        false,
                                    ),
                                )
                            }
                        }
                    }
                    TradeAllowanceChargeTypeSerializerState::Reason(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state =
                                    TradeAllowanceChargeTypeSerializerState::CategoryTradeTax(
                                        WithSerializer::serializer(
                                            &self.value.category_trade_tax,
                                            Some("ram:CategoryTradeTax"),
                                            false,
                                        )?,
                                    )
                            }
                        }
                    }
                    TradeAllowanceChargeTypeSerializerState::CategoryTradeTax(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = TradeAllowanceChargeTypeSerializerState::End__,
                        }
                    }
                    TradeAllowanceChargeTypeSerializerState::End__ => {
                        *self.state = TradeAllowanceChargeTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    TradeAllowanceChargeTypeSerializerState::Done__ => return Ok(None),
                    TradeAllowanceChargeTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for TradeAllowanceChargeTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = TradeAllowanceChargeTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct TradePaymentTermsTypeSerializer<'ser> {
        pub(super) value: &'ser super::TradePaymentTermsType,
        pub(super) state: Box<TradePaymentTermsTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum TradePaymentTermsTypeSerializerState<'ser> {
        Init__,
        Description(IterSerializer<'ser, Option<&'ser super::TextType>, super::TextType>),
        DueDateDateTime(
            IterSerializer<'ser, Option<&'ser super::DateTimeType>, super::DateTimeType>,
        ),
        DirectDebitMandateId(IterSerializer<'ser, Option<&'ser super::IdType>, super::IdType>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> TradePaymentTermsTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    TradePaymentTermsTypeSerializerState::Init__ => {
                        *self.state =
                            TradePaymentTermsTypeSerializerState::Description(IterSerializer::new(
                                self.value.description.as_ref(),
                                Some("ram:Description"),
                                false,
                            ));
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:rsm"[..], &super::NS_RSM[..]));
                            bytes.push_attribute((&b"xmlns:qdt"[..], &super::NS_QDT[..]));
                            bytes.push_attribute((&b"xmlns:ram"[..], &super::NS_RAM[..]));
                            bytes.push_attribute((&b"xmlns:udt"[..], &super::NS_UDT[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    TradePaymentTermsTypeSerializerState::Description(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = TradePaymentTermsTypeSerializerState::DueDateDateTime(
                                    IterSerializer::new(
                                        self.value.due_date_date_time.as_ref(),
                                        Some("ram:DueDateDateTime"),
                                        false,
                                    ),
                                )
                            }
                        }
                    }
                    TradePaymentTermsTypeSerializerState::DueDateDateTime(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state =
                                    TradePaymentTermsTypeSerializerState::DirectDebitMandateId(
                                        IterSerializer::new(
                                            self.value.direct_debit_mandate_id.as_ref(),
                                            Some("ram:DirectDebitMandateID"),
                                            false,
                                        ),
                                    )
                            }
                        }
                    }
                    TradePaymentTermsTypeSerializerState::DirectDebitMandateId(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = TradePaymentTermsTypeSerializerState::End__,
                        }
                    }
                    TradePaymentTermsTypeSerializerState::End__ => {
                        *self.state = TradePaymentTermsTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    TradePaymentTermsTypeSerializerState::Done__ => return Ok(None),
                    TradePaymentTermsTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for TradePaymentTermsTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = TradePaymentTermsTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct TradeSettlementHeaderMonetarySummationTypeSerializer<'ser> {
        pub(super) value: &'ser super::TradeSettlementHeaderMonetarySummationType,
        pub(super) state: Box<TradeSettlementHeaderMonetarySummationTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum TradeSettlementHeaderMonetarySummationTypeSerializerState<'ser> {
        Init__,
        LineTotalAmount(<super::AmountType as WithSerializer>::Serializer<'ser>),
        ChargeTotalAmount(IterSerializer<'ser, Option<&'ser super::AmountType>, super::AmountType>),
        AllowanceTotalAmount(
            IterSerializer<'ser, Option<&'ser super::AmountType>, super::AmountType>,
        ),
        TaxBasisTotalAmount(<super::AmountType as WithSerializer>::Serializer<'ser>),
        TaxTotalAmount(IterSerializer<'ser, &'ser [super::AmountType], super::AmountType>),
        GrandTotalAmount(<super::AmountType as WithSerializer>::Serializer<'ser>),
        TotalPrepaidAmount(
            IterSerializer<'ser, Option<&'ser super::AmountType>, super::AmountType>,
        ),
        DuePayableAmount(<super::AmountType as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> TradeSettlementHeaderMonetarySummationTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match & mut * self . state { TradeSettlementHeaderMonetarySummationTypeSerializerState :: Init__ => { * self . state = TradeSettlementHeaderMonetarySummationTypeSerializerState :: LineTotalAmount (WithSerializer :: serializer (& self . value . line_total_amount , Some ("ram:LineTotalAmount") , false) ?) ; let mut bytes = BytesStart :: new (self . name) ; if self . is_root { bytes . push_attribute ((& b"xmlns:rsm" [..] , & super :: NS_RSM [..])) ; bytes . push_attribute ((& b"xmlns:qdt" [..] , & super :: NS_QDT [..])) ; bytes . push_attribute ((& b"xmlns:ram" [..] , & super :: NS_RAM [..])) ; bytes . push_attribute ((& b"xmlns:udt" [..] , & super :: NS_UDT [..])) ; } return Ok (Some (Event :: Start (bytes))) } TradeSettlementHeaderMonetarySummationTypeSerializerState :: LineTotalAmount (x) => match x . next () . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = TradeSettlementHeaderMonetarySummationTypeSerializerState :: ChargeTotalAmount (IterSerializer :: new (self . value . charge_total_amount . as_ref () , Some ("ram:ChargeTotalAmount") , false)) , } TradeSettlementHeaderMonetarySummationTypeSerializerState :: ChargeTotalAmount (x) => match x . next () . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = TradeSettlementHeaderMonetarySummationTypeSerializerState :: AllowanceTotalAmount (IterSerializer :: new (self . value . allowance_total_amount . as_ref () , Some ("ram:AllowanceTotalAmount") , false)) , } TradeSettlementHeaderMonetarySummationTypeSerializerState :: AllowanceTotalAmount (x) => match x . next () . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = TradeSettlementHeaderMonetarySummationTypeSerializerState :: TaxBasisTotalAmount (WithSerializer :: serializer (& self . value . tax_basis_total_amount , Some ("ram:TaxBasisTotalAmount") , false) ?) , } TradeSettlementHeaderMonetarySummationTypeSerializerState :: TaxBasisTotalAmount (x) => match x . next () . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = TradeSettlementHeaderMonetarySummationTypeSerializerState :: TaxTotalAmount (IterSerializer :: new (& self . value . tax_total_amount [..] , Some ("ram:TaxTotalAmount") , false)) , } TradeSettlementHeaderMonetarySummationTypeSerializerState :: TaxTotalAmount (x) => match x . next () . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = TradeSettlementHeaderMonetarySummationTypeSerializerState :: GrandTotalAmount (WithSerializer :: serializer (& self . value . grand_total_amount , Some ("ram:GrandTotalAmount") , false) ?) , } TradeSettlementHeaderMonetarySummationTypeSerializerState :: GrandTotalAmount (x) => match x . next () . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = TradeSettlementHeaderMonetarySummationTypeSerializerState :: TotalPrepaidAmount (IterSerializer :: new (self . value . total_prepaid_amount . as_ref () , Some ("ram:TotalPrepaidAmount") , false)) , } TradeSettlementHeaderMonetarySummationTypeSerializerState :: TotalPrepaidAmount (x) => match x . next () . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = TradeSettlementHeaderMonetarySummationTypeSerializerState :: DuePayableAmount (WithSerializer :: serializer (& self . value . due_payable_amount , Some ("ram:DuePayableAmount") , false) ?) , } TradeSettlementHeaderMonetarySummationTypeSerializerState :: DuePayableAmount (x) => match x . next () . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = TradeSettlementHeaderMonetarySummationTypeSerializerState :: End__ , } TradeSettlementHeaderMonetarySummationTypeSerializerState :: End__ => { * self . state = TradeSettlementHeaderMonetarySummationTypeSerializerState :: Done__ ; return Ok (Some (Event :: End (BytesEnd :: new (self . name)))) ; } TradeSettlementHeaderMonetarySummationTypeSerializerState :: Done__ => return Ok (None) , TradeSettlementHeaderMonetarySummationTypeSerializerState :: Phantom__ (_) => unreachable ! () , }
            }
        }
    }
    impl<'ser> Iterator for TradeSettlementHeaderMonetarySummationTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = TradeSettlementHeaderMonetarySummationTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct TradeAccountingAccountTypeSerializer<'ser> {
        pub(super) value: &'ser super::TradeAccountingAccountType,
        pub(super) state: Box<TradeAccountingAccountTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum TradeAccountingAccountTypeSerializerState<'ser> {
        Init__,
        Id(<super::IdType as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> TradeAccountingAccountTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    TradeAccountingAccountTypeSerializerState::Init__ => {
                        *self.state = TradeAccountingAccountTypeSerializerState::Id(
                            WithSerializer::serializer(&self.value.id, Some("ram:ID"), false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:rsm"[..], &super::NS_RSM[..]));
                            bytes.push_attribute((&b"xmlns:qdt"[..], &super::NS_QDT[..]));
                            bytes.push_attribute((&b"xmlns:ram"[..], &super::NS_RAM[..]));
                            bytes.push_attribute((&b"xmlns:udt"[..], &super::NS_UDT[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    TradeAccountingAccountTypeSerializerState::Id(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = TradeAccountingAccountTypeSerializerState::End__,
                        }
                    }
                    TradeAccountingAccountTypeSerializerState::End__ => {
                        *self.state = TradeAccountingAccountTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    TradeAccountingAccountTypeSerializerState::Done__ => return Ok(None),
                    TradeAccountingAccountTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for TradeAccountingAccountTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = TradeAccountingAccountTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct LegalOrganizationTypeSerializer<'ser> {
        pub(super) value: &'ser super::LegalOrganizationType,
        pub(super) state: Box<LegalOrganizationTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum LegalOrganizationTypeSerializerState<'ser> {
        Init__,
        Id(IterSerializer<'ser, Option<&'ser super::IdType>, super::IdType>),
        TradingBusinessName(IterSerializer<'ser, Option<&'ser super::TextType>, super::TextType>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> LegalOrganizationTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    LegalOrganizationTypeSerializerState::Init__ => {
                        *self.state = LegalOrganizationTypeSerializerState::Id(
                            IterSerializer::new(self.value.id.as_ref(), Some("ram:ID"), false),
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:rsm"[..], &super::NS_RSM[..]));
                            bytes.push_attribute((&b"xmlns:qdt"[..], &super::NS_QDT[..]));
                            bytes.push_attribute((&b"xmlns:ram"[..], &super::NS_RAM[..]));
                            bytes.push_attribute((&b"xmlns:udt"[..], &super::NS_UDT[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    LegalOrganizationTypeSerializerState::Id(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = LegalOrganizationTypeSerializerState::TradingBusinessName(
                                IterSerializer::new(
                                    self.value.trading_business_name.as_ref(),
                                    Some("ram:TradingBusinessName"),
                                    false,
                                ),
                            )
                        }
                    },
                    LegalOrganizationTypeSerializerState::TradingBusinessName(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = LegalOrganizationTypeSerializerState::End__,
                        }
                    }
                    LegalOrganizationTypeSerializerState::End__ => {
                        *self.state = LegalOrganizationTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    LegalOrganizationTypeSerializerState::Done__ => return Ok(None),
                    LegalOrganizationTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for LegalOrganizationTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = LegalOrganizationTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct TradeAddressTypeSerializer<'ser> {
        pub(super) value: &'ser super::TradeAddressType,
        pub(super) state: Box<TradeAddressTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum TradeAddressTypeSerializerState<'ser> {
        Init__,
        PostcodeCode(IterSerializer<'ser, Option<&'ser super::CodeType>, super::CodeType>),
        LineOne(IterSerializer<'ser, Option<&'ser super::TextType>, super::TextType>),
        LineTwo(IterSerializer<'ser, Option<&'ser super::TextType>, super::TextType>),
        LineThree(IterSerializer<'ser, Option<&'ser super::TextType>, super::TextType>),
        CityName(IterSerializer<'ser, Option<&'ser super::TextType>, super::TextType>),
        CountryId(<super::CountryIdType as WithSerializer>::Serializer<'ser>),
        CountrySubDivisionName(
            IterSerializer<'ser, Option<&'ser super::TextType>, super::TextType>,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> TradeAddressTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    TradeAddressTypeSerializerState::Init__ => {
                        *self.state =
                            TradeAddressTypeSerializerState::PostcodeCode(IterSerializer::new(
                                self.value.postcode_code.as_ref(),
                                Some("ram:PostcodeCode"),
                                false,
                            ));
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:rsm"[..], &super::NS_RSM[..]));
                            bytes.push_attribute((&b"xmlns:qdt"[..], &super::NS_QDT[..]));
                            bytes.push_attribute((&b"xmlns:ram"[..], &super::NS_RAM[..]));
                            bytes.push_attribute((&b"xmlns:udt"[..], &super::NS_UDT[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    TradeAddressTypeSerializerState::PostcodeCode(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state =
                                    TradeAddressTypeSerializerState::LineOne(IterSerializer::new(
                                        self.value.line_one.as_ref(),
                                        Some("ram:LineOne"),
                                        false,
                                    ))
                            }
                        }
                    }
                    TradeAddressTypeSerializerState::LineOne(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state =
                                TradeAddressTypeSerializerState::LineTwo(IterSerializer::new(
                                    self.value.line_two.as_ref(),
                                    Some("ram:LineTwo"),
                                    false,
                                ))
                        }
                    },
                    TradeAddressTypeSerializerState::LineTwo(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state =
                                TradeAddressTypeSerializerState::LineThree(IterSerializer::new(
                                    self.value.line_three.as_ref(),
                                    Some("ram:LineThree"),
                                    false,
                                ))
                        }
                    },
                    TradeAddressTypeSerializerState::LineThree(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state =
                                TradeAddressTypeSerializerState::CityName(IterSerializer::new(
                                    self.value.city_name.as_ref(),
                                    Some("ram:CityName"),
                                    false,
                                ))
                        }
                    },
                    TradeAddressTypeSerializerState::CityName(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = TradeAddressTypeSerializerState::CountryId(
                                WithSerializer::serializer(
                                    &self.value.country_id,
                                    Some("ram:CountryID"),
                                    false,
                                )?,
                            )
                        }
                    },
                    TradeAddressTypeSerializerState::CountryId(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = TradeAddressTypeSerializerState::CountrySubDivisionName(
                                IterSerializer::new(
                                    self.value.country_sub_division_name.as_ref(),
                                    Some("ram:CountrySubDivisionName"),
                                    false,
                                ),
                            )
                        }
                    },
                    TradeAddressTypeSerializerState::CountrySubDivisionName(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = TradeAddressTypeSerializerState::End__,
                        }
                    }
                    TradeAddressTypeSerializerState::End__ => {
                        *self.state = TradeAddressTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    TradeAddressTypeSerializerState::Done__ => return Ok(None),
                    TradeAddressTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for TradeAddressTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = TradeAddressTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct UniversalCommunicationTypeSerializer<'ser> {
        pub(super) value: &'ser super::UniversalCommunicationType,
        pub(super) state: Box<UniversalCommunicationTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum UniversalCommunicationTypeSerializerState<'ser> {
        Init__,
        Uriid(<super::IdType as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> UniversalCommunicationTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    UniversalCommunicationTypeSerializerState::Init__ => {
                        *self.state = UniversalCommunicationTypeSerializerState::Uriid(
                            WithSerializer::serializer(
                                &self.value.uriid,
                                Some("ram:URIID"),
                                false,
                            )?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:rsm"[..], &super::NS_RSM[..]));
                            bytes.push_attribute((&b"xmlns:qdt"[..], &super::NS_QDT[..]));
                            bytes.push_attribute((&b"xmlns:ram"[..], &super::NS_RAM[..]));
                            bytes.push_attribute((&b"xmlns:udt"[..], &super::NS_UDT[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    UniversalCommunicationTypeSerializerState::Uriid(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = UniversalCommunicationTypeSerializerState::End__,
                        }
                    }
                    UniversalCommunicationTypeSerializerState::End__ => {
                        *self.state = UniversalCommunicationTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    UniversalCommunicationTypeSerializerState::Done__ => return Ok(None),
                    UniversalCommunicationTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for UniversalCommunicationTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = UniversalCommunicationTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct TaxRegistrationTypeSerializer<'ser> {
        pub(super) value: &'ser super::TaxRegistrationType,
        pub(super) state: Box<TaxRegistrationTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum TaxRegistrationTypeSerializerState<'ser> {
        Init__,
        Id(<super::IdType as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> TaxRegistrationTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    TaxRegistrationTypeSerializerState::Init__ => {
                        *self.state = TaxRegistrationTypeSerializerState::Id(
                            WithSerializer::serializer(&self.value.id, Some("ram:ID"), false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:rsm"[..], &super::NS_RSM[..]));
                            bytes.push_attribute((&b"xmlns:qdt"[..], &super::NS_QDT[..]));
                            bytes.push_attribute((&b"xmlns:ram"[..], &super::NS_RAM[..]));
                            bytes.push_attribute((&b"xmlns:udt"[..], &super::NS_UDT[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    TaxRegistrationTypeSerializerState::Id(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = TaxRegistrationTypeSerializerState::End__,
                    },
                    TaxRegistrationTypeSerializerState::End__ => {
                        *self.state = TaxRegistrationTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    TaxRegistrationTypeSerializerState::Done__ => return Ok(None),
                    TaxRegistrationTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for TaxRegistrationTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = TaxRegistrationTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct FormattedDateTimeTypeSerializer<'ser> {
        pub(super) value: &'ser super::FormattedDateTimeType,
        pub(super) state: Box<FormattedDateTimeTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum FormattedDateTimeTypeSerializerState<'ser> {
        Init__,
        DateTimeString(
            <super::FormattedDateTimeTypeDateTimeStringType as WithSerializer>::Serializer<'ser>,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> FormattedDateTimeTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    FormattedDateTimeTypeSerializerState::Init__ => {
                        *self.state = FormattedDateTimeTypeSerializerState::DateTimeString(
                            WithSerializer::serializer(
                                &self.value.date_time_string,
                                Some("qdt:DateTimeString"),
                                false,
                            )?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:rsm"[..], &super::NS_RSM[..]));
                            bytes.push_attribute((&b"xmlns:qdt"[..], &super::NS_QDT[..]));
                            bytes.push_attribute((&b"xmlns:ram"[..], &super::NS_RAM[..]));
                            bytes.push_attribute((&b"xmlns:udt"[..], &super::NS_UDT[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    FormattedDateTimeTypeSerializerState::DateTimeString(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = FormattedDateTimeTypeSerializerState::End__,
                        }
                    }
                    FormattedDateTimeTypeSerializerState::End__ => {
                        *self.state = FormattedDateTimeTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    FormattedDateTimeTypeSerializerState::Done__ => return Ok(None),
                    FormattedDateTimeTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for FormattedDateTimeTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = FormattedDateTimeTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct PaymentMeansCodeTypeSerializer<'ser> {
        pub(super) value: &'ser super::PaymentMeansCodeType,
        pub(super) state: Box<PaymentMeansCodeTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum PaymentMeansCodeTypeSerializerState<'ser> {
        Init__,
        Content__(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> PaymentMeansCodeTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    PaymentMeansCodeTypeSerializerState::Init__ => {
                        *self.state = PaymentMeansCodeTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:rsm"[..], &super::NS_RSM[..]));
                            bytes.push_attribute((&b"xmlns:qdt"[..], &super::NS_QDT[..]));
                            bytes.push_attribute((&b"xmlns:ram"[..], &super::NS_RAM[..]));
                            bytes.push_attribute((&b"xmlns:udt"[..], &super::NS_UDT[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    PaymentMeansCodeTypeSerializerState::Content__(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = PaymentMeansCodeTypeSerializerState::End__,
                        }
                    }
                    PaymentMeansCodeTypeSerializerState::End__ => {
                        *self.state = PaymentMeansCodeTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    PaymentMeansCodeTypeSerializerState::Done__ => return Ok(None),
                    PaymentMeansCodeTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for PaymentMeansCodeTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = PaymentMeansCodeTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct DebtorFinancialAccountTypeSerializer<'ser> {
        pub(super) value: &'ser super::DebtorFinancialAccountType,
        pub(super) state: Box<DebtorFinancialAccountTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum DebtorFinancialAccountTypeSerializerState<'ser> {
        Init__,
        Ibanid(<super::IdType as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> DebtorFinancialAccountTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    DebtorFinancialAccountTypeSerializerState::Init__ => {
                        *self.state = DebtorFinancialAccountTypeSerializerState::Ibanid(
                            WithSerializer::serializer(
                                &self.value.ibanid,
                                Some("ram:IBANID"),
                                false,
                            )?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:rsm"[..], &super::NS_RSM[..]));
                            bytes.push_attribute((&b"xmlns:qdt"[..], &super::NS_QDT[..]));
                            bytes.push_attribute((&b"xmlns:ram"[..], &super::NS_RAM[..]));
                            bytes.push_attribute((&b"xmlns:udt"[..], &super::NS_UDT[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    DebtorFinancialAccountTypeSerializerState::Ibanid(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = DebtorFinancialAccountTypeSerializerState::End__,
                        }
                    }
                    DebtorFinancialAccountTypeSerializerState::End__ => {
                        *self.state = DebtorFinancialAccountTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    DebtorFinancialAccountTypeSerializerState::Done__ => return Ok(None),
                    DebtorFinancialAccountTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for DebtorFinancialAccountTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = DebtorFinancialAccountTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct CreditorFinancialAccountTypeSerializer<'ser> {
        pub(super) value: &'ser super::CreditorFinancialAccountType,
        pub(super) state: Box<CreditorFinancialAccountTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum CreditorFinancialAccountTypeSerializerState<'ser> {
        Init__,
        Ibanid(IterSerializer<'ser, Option<&'ser super::IdType>, super::IdType>),
        ProprietaryId(IterSerializer<'ser, Option<&'ser super::IdType>, super::IdType>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> CreditorFinancialAccountTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    CreditorFinancialAccountTypeSerializerState::Init__ => {
                        *self.state = CreditorFinancialAccountTypeSerializerState::Ibanid(
                            IterSerializer::new(
                                self.value.ibanid.as_ref(),
                                Some("ram:IBANID"),
                                false,
                            ),
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:rsm"[..], &super::NS_RSM[..]));
                            bytes.push_attribute((&b"xmlns:qdt"[..], &super::NS_QDT[..]));
                            bytes.push_attribute((&b"xmlns:ram"[..], &super::NS_RAM[..]));
                            bytes.push_attribute((&b"xmlns:udt"[..], &super::NS_UDT[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    CreditorFinancialAccountTypeSerializerState::Ibanid(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state =
                                    CreditorFinancialAccountTypeSerializerState::ProprietaryId(
                                        IterSerializer::new(
                                            self.value.proprietary_id.as_ref(),
                                            Some("ram:ProprietaryID"),
                                            false,
                                        ),
                                    )
                            }
                        }
                    }
                    CreditorFinancialAccountTypeSerializerState::ProprietaryId(x) => match x
                        .next()
                        .transpose()?
                    {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = CreditorFinancialAccountTypeSerializerState::End__,
                    },
                    CreditorFinancialAccountTypeSerializerState::End__ => {
                        *self.state = CreditorFinancialAccountTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    CreditorFinancialAccountTypeSerializerState::Done__ => return Ok(None),
                    CreditorFinancialAccountTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for CreditorFinancialAccountTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = CreditorFinancialAccountTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct AmountTypeSerializer<'ser> {
        pub(super) value: &'ser super::AmountType,
        pub(super) state: Box<AmountTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum AmountTypeSerializerState<'ser> {
        Init__,
        Content__(<f64 as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> AmountTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    AmountTypeSerializerState::Init__ => {
                        *self.state = AmountTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:rsm"[..], &super::NS_RSM[..]));
                            bytes.push_attribute((&b"xmlns:qdt"[..], &super::NS_QDT[..]));
                            bytes.push_attribute((&b"xmlns:ram"[..], &super::NS_RAM[..]));
                            bytes.push_attribute((&b"xmlns:udt"[..], &super::NS_UDT[..]));
                        }
                        write_attrib_opt(&mut bytes, "udt:currencyID", &self.value.currency_id)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    AmountTypeSerializerState::Content__(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = AmountTypeSerializerState::End__,
                    },
                    AmountTypeSerializerState::End__ => {
                        *self.state = AmountTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    AmountTypeSerializerState::Done__ => return Ok(None),
                    AmountTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for AmountTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = AmountTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct TaxTypeCodeTypeSerializer<'ser> {
        pub(super) value: &'ser super::TaxTypeCodeType,
        pub(super) state: Box<TaxTypeCodeTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum TaxTypeCodeTypeSerializerState<'ser> {
        Init__,
        Content__(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> TaxTypeCodeTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    TaxTypeCodeTypeSerializerState::Init__ => {
                        *self.state = TaxTypeCodeTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:rsm"[..], &super::NS_RSM[..]));
                            bytes.push_attribute((&b"xmlns:qdt"[..], &super::NS_QDT[..]));
                            bytes.push_attribute((&b"xmlns:ram"[..], &super::NS_RAM[..]));
                            bytes.push_attribute((&b"xmlns:udt"[..], &super::NS_UDT[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    TaxTypeCodeTypeSerializerState::Content__(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = TaxTypeCodeTypeSerializerState::End__,
                    },
                    TaxTypeCodeTypeSerializerState::End__ => {
                        *self.state = TaxTypeCodeTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    TaxTypeCodeTypeSerializerState::Done__ => return Ok(None),
                    TaxTypeCodeTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for TaxTypeCodeTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = TaxTypeCodeTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct TaxCategoryCodeTypeSerializer<'ser> {
        pub(super) value: &'ser super::TaxCategoryCodeType,
        pub(super) state: Box<TaxCategoryCodeTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum TaxCategoryCodeTypeSerializerState<'ser> {
        Init__,
        Content__(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> TaxCategoryCodeTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    TaxCategoryCodeTypeSerializerState::Init__ => {
                        *self.state = TaxCategoryCodeTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:rsm"[..], &super::NS_RSM[..]));
                            bytes.push_attribute((&b"xmlns:qdt"[..], &super::NS_QDT[..]));
                            bytes.push_attribute((&b"xmlns:ram"[..], &super::NS_RAM[..]));
                            bytes.push_attribute((&b"xmlns:udt"[..], &super::NS_UDT[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    TaxCategoryCodeTypeSerializerState::Content__(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = TaxCategoryCodeTypeSerializerState::End__,
                        }
                    }
                    TaxCategoryCodeTypeSerializerState::End__ => {
                        *self.state = TaxCategoryCodeTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    TaxCategoryCodeTypeSerializerState::Done__ => return Ok(None),
                    TaxCategoryCodeTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for TaxCategoryCodeTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = TaxCategoryCodeTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct TimeReferenceCodeTypeSerializer<'ser> {
        pub(super) value: &'ser super::TimeReferenceCodeType,
        pub(super) state: Box<TimeReferenceCodeTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum TimeReferenceCodeTypeSerializerState<'ser> {
        Init__,
        Content__(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> TimeReferenceCodeTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    TimeReferenceCodeTypeSerializerState::Init__ => {
                        *self.state = TimeReferenceCodeTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:rsm"[..], &super::NS_RSM[..]));
                            bytes.push_attribute((&b"xmlns:qdt"[..], &super::NS_QDT[..]));
                            bytes.push_attribute((&b"xmlns:ram"[..], &super::NS_RAM[..]));
                            bytes.push_attribute((&b"xmlns:udt"[..], &super::NS_UDT[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    TimeReferenceCodeTypeSerializerState::Content__(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = TimeReferenceCodeTypeSerializerState::End__,
                        }
                    }
                    TimeReferenceCodeTypeSerializerState::End__ => {
                        *self.state = TimeReferenceCodeTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    TimeReferenceCodeTypeSerializerState::Done__ => return Ok(None),
                    TimeReferenceCodeTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for TimeReferenceCodeTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = TimeReferenceCodeTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct PercentTypeSerializer<'ser> {
        pub(super) value: &'ser super::PercentType,
        pub(super) state: Box<PercentTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum PercentTypeSerializerState<'ser> {
        Init__,
        Content__(<f64 as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> PercentTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    PercentTypeSerializerState::Init__ => {
                        *self.state = PercentTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:rsm"[..], &super::NS_RSM[..]));
                            bytes.push_attribute((&b"xmlns:qdt"[..], &super::NS_QDT[..]));
                            bytes.push_attribute((&b"xmlns:ram"[..], &super::NS_RAM[..]));
                            bytes.push_attribute((&b"xmlns:udt"[..], &super::NS_UDT[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    PercentTypeSerializerState::Content__(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = PercentTypeSerializerState::End__,
                    },
                    PercentTypeSerializerState::End__ => {
                        *self.state = PercentTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    PercentTypeSerializerState::Done__ => return Ok(None),
                    PercentTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for PercentTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = PercentTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct IndicatorTypeSerializer<'ser> {
        pub(super) value: &'ser super::IndicatorType,
        pub(super) state: Box<IndicatorTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum IndicatorTypeSerializerState<'ser> {
        Init__,
        Content__(<super::IndicatorTypeContent as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> IndicatorTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    IndicatorTypeSerializerState::Init__ => {
                        *self.state = IndicatorTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:rsm"[..], &super::NS_RSM[..]));
                            bytes.push_attribute((&b"xmlns:qdt"[..], &super::NS_QDT[..]));
                            bytes.push_attribute((&b"xmlns:ram"[..], &super::NS_RAM[..]));
                            bytes.push_attribute((&b"xmlns:udt"[..], &super::NS_UDT[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    IndicatorTypeSerializerState::Content__(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = IndicatorTypeSerializerState::End__,
                    },
                    IndicatorTypeSerializerState::End__ => {
                        *self.state = IndicatorTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    IndicatorTypeSerializerState::Done__ => return Ok(None),
                    IndicatorTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for IndicatorTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = IndicatorTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct IndicatorTypeContentSerializer<'ser> {
        pub(super) value: &'ser super::IndicatorTypeContent,
        pub(super) state: Box<IndicatorTypeContentSerializerState<'ser>>,
    }
    #[derive(Debug)]
    pub(super) enum IndicatorTypeContentSerializerState<'ser> {
        Init__,
        Indicator(<bool as WithSerializer>::Serializer<'ser>),
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> IndicatorTypeContentSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    IndicatorTypeContentSerializerState::Init__ => match self.value {
                        super::IndicatorTypeContent::Indicator(x) => {
                            *self.state = IndicatorTypeContentSerializerState::Indicator(
                                WithSerializer::serializer(x, Some("udt:Indicator"), false)?,
                            )
                        }
                    },
                    IndicatorTypeContentSerializerState::Indicator(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = IndicatorTypeContentSerializerState::Done__,
                        }
                    }
                    IndicatorTypeContentSerializerState::Done__ => return Ok(None),
                    IndicatorTypeContentSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for IndicatorTypeContentSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = IndicatorTypeContentSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct AllowanceChargeReasonCodeTypeSerializer<'ser> {
        pub(super) value: &'ser super::AllowanceChargeReasonCodeType,
        pub(super) state: Box<AllowanceChargeReasonCodeTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum AllowanceChargeReasonCodeTypeSerializerState<'ser> {
        Init__,
        Content__(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> AllowanceChargeReasonCodeTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    AllowanceChargeReasonCodeTypeSerializerState::Init__ => {
                        *self.state = AllowanceChargeReasonCodeTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:rsm"[..], &super::NS_RSM[..]));
                            bytes.push_attribute((&b"xmlns:qdt"[..], &super::NS_QDT[..]));
                            bytes.push_attribute((&b"xmlns:ram"[..], &super::NS_RAM[..]));
                            bytes.push_attribute((&b"xmlns:udt"[..], &super::NS_UDT[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    AllowanceChargeReasonCodeTypeSerializerState::Content__(x) => match x
                        .next()
                        .transpose()?
                    {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = AllowanceChargeReasonCodeTypeSerializerState::End__,
                    },
                    AllowanceChargeReasonCodeTypeSerializerState::End__ => {
                        *self.state = AllowanceChargeReasonCodeTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    AllowanceChargeReasonCodeTypeSerializerState::Done__ => return Ok(None),
                    AllowanceChargeReasonCodeTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for AllowanceChargeReasonCodeTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = AllowanceChargeReasonCodeTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct CountryIdTypeSerializer<'ser> {
        pub(super) value: &'ser super::CountryIdType,
        pub(super) state: Box<CountryIdTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum CountryIdTypeSerializerState<'ser> {
        Init__,
        Content__(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> CountryIdTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    CountryIdTypeSerializerState::Init__ => {
                        *self.state = CountryIdTypeSerializerState::Content__(
                            WithSerializer::serializer(&self.value.content, None, false)?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:rsm"[..], &super::NS_RSM[..]));
                            bytes.push_attribute((&b"xmlns:qdt"[..], &super::NS_QDT[..]));
                            bytes.push_attribute((&b"xmlns:ram"[..], &super::NS_RAM[..]));
                            bytes.push_attribute((&b"xmlns:udt"[..], &super::NS_UDT[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    CountryIdTypeSerializerState::Content__(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = CountryIdTypeSerializerState::End__,
                    },
                    CountryIdTypeSerializerState::End__ => {
                        *self.state = CountryIdTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    CountryIdTypeSerializerState::Done__ => return Ok(None),
                    CountryIdTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for CountryIdTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = CountryIdTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct FormattedDateTimeTypeDateTimeStringTypeSerializer<'ser> {
        pub(super) value: &'ser super::FormattedDateTimeTypeDateTimeStringType,
        pub(super) state: Box<FormattedDateTimeTypeDateTimeStringTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum FormattedDateTimeTypeDateTimeStringTypeSerializerState<'ser> {
        Init__,
        Content__(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> FormattedDateTimeTypeDateTimeStringTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    FormattedDateTimeTypeDateTimeStringTypeSerializerState::Init__ => {
                        *self.state =
                            FormattedDateTimeTypeDateTimeStringTypeSerializerState::Content__(
                                WithSerializer::serializer(&self.value.content, None, false)?,
                            );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:rsm"[..], &super::NS_RSM[..]));
                            bytes.push_attribute((&b"xmlns:qdt"[..], &super::NS_QDT[..]));
                            bytes.push_attribute((&b"xmlns:ram"[..], &super::NS_RAM[..]));
                            bytes.push_attribute((&b"xmlns:udt"[..], &super::NS_UDT[..]));
                        }
                        write_attrib(&mut bytes, "qdt:format", &self.value.format)?;
                        return Ok(Some(Event::Start(bytes)));
                    }
                    FormattedDateTimeTypeDateTimeStringTypeSerializerState::Content__(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state =
                                    FormattedDateTimeTypeDateTimeStringTypeSerializerState::End__
                            }
                        }
                    }
                    FormattedDateTimeTypeDateTimeStringTypeSerializerState::End__ => {
                        *self.state =
                            FormattedDateTimeTypeDateTimeStringTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    FormattedDateTimeTypeDateTimeStringTypeSerializerState::Done__ => {
                        return Ok(None)
                    }
                    FormattedDateTimeTypeDateTimeStringTypeSerializerState::Phantom__(_) => {
                        unreachable!()
                    }
                }
            }
        }
    }
    impl<'ser> Iterator for FormattedDateTimeTypeDateTimeStringTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = FormattedDateTimeTypeDateTimeStringTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
