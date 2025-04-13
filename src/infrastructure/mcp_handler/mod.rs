use std::sync::Arc;

use rmcp::{
    Error as McpError, RoleServer, ServerHandler, const_string, model::*, service::RequestContext,
    tool,
};
use serde_json::json;

use crate::{
    application::use_cases::{
        bitcoin_flow::BitcoinFlowUseCase, cash_flow::CashFlowUseCase, debt_radar::DebtRadarUseCase,
        spending_scanner::SpendingScannerUseCase,
    },
    domain::value_objects::{
        bitcoin_flow::{BuyBitcoinModel, SellBitcoinModel},
        cash_flow::{RecordCashFlowModel, RecordCashFlowWithDateModel},
        debt_radar::{PaidDebtModel, RecordDebtModel, WhoOwesMeModel},
        spending_scanner::SpendingScannerFilter,
    },
};

#[derive(Clone)]
pub struct MCPHandler {
    cash_flow_use_case: Arc<CashFlowUseCase>,
    spending_scanner_use_case: Arc<SpendingScannerUseCase>,
    debt_radar_use_case: Arc<DebtRadarUseCase>,
    bitcoin_flow_use_case: Arc<BitcoinFlowUseCase>,
}

#[tool(tool_box)]
impl MCPHandler {
    pub fn new(
        cash_flow_use_case: Arc<CashFlowUseCase>,
        spending_scanner_use_case: Arc<SpendingScannerUseCase>,
        debt_radar_use_case: Arc<DebtRadarUseCase>,
        bitcoin_flow_use_case: Arc<BitcoinFlowUseCase>,
    ) -> Self {
        Self {
            cash_flow_use_case,
            spending_scanner_use_case,
            debt_radar_use_case,
            bitcoin_flow_use_case,
        }
    }

    fn _create_resource_text(&self, uri: &str, name: &str) -> Resource {
        RawResource::new(uri, name.to_string()).no_annotation()
    }

    #[tool(description = "Record a cash flow ledger transaction")]
    pub async fn record_cash_flow(
        &self,
        #[tool(aggr)] record_cash_flow_model: RecordCashFlowModel,
    ) -> Result<CallToolResult, McpError> {
        match self.cash_flow_use_case.record(record_cash_flow_model).await {
            Ok(id) => Ok(CallToolResult::success(vec![Content::text(format!(
                "Cash flow ledger transaction recorded successfully: id: {}",
                id
            ))])),
            Err(e) => Err(McpError::internal_error(e.to_string(), None)),
        }
    }

    #[tool(description = "Record a cash flow ledger transaction with date")]
    pub async fn record_cash_flow_with_date(
        &self,
        #[tool(aggr)] record_cash_flow_with_date_model: RecordCashFlowWithDateModel,
    ) -> Result<CallToolResult, McpError> {
        match self
            .cash_flow_use_case
            .record_with_date(record_cash_flow_with_date_model)
            .await
        {
            Ok(id) => Ok(CallToolResult::success(vec![Content::text(format!(
                "Cash flow ledger transaction recorded successfully: id: {}",
                id
            ))])),
            Err(e) => Err(McpError::internal_error(e.to_string(), None)),
        }
    }

    #[tool(
        description = "See how much money you have spent by date range: today, this month, this year, or lifetime."
    )]
    pub async fn spending_scanner(
        &self,
        #[tool(aggr)] spending_scanner_filter: SpendingScannerFilter,
    ) -> Result<CallToolResult, McpError> {
        match self
            .spending_scanner_use_case
            .scan(spending_scanner_filter)
            .await
        {
            Ok(results) => {
                if let Ok(res_json) = Content::json(results) {
                    Ok(CallToolResult::success(vec![res_json]))
                } else {
                    Err(McpError::internal_error(
                        "Failed to convert results to JSON".to_string(),
                        None,
                    ))
                }
            }
            Err(e) => Err(McpError::internal_error(e.to_string(), None)),
        }
    }

    #[tool(
        description = "Visualize your spending by category: today, this month, this year, or lifetime."
    )]
    pub async fn spending_visualizer(
        &self,
        #[tool(aggr)] spending_scanner_filter: SpendingScannerFilter,
    ) -> Result<CallToolResult, McpError> {
        match self
            .spending_scanner_use_case
            .visualize(spending_scanner_filter)
            .await
        {
            Ok(results) => {
                if let Ok(res_json) = Content::json(results) {
                    Ok(CallToolResult::success(vec![res_json]))
                } else {
                    Err(McpError::internal_error(
                        "Failed to convert results to JSON".to_string(),
                        None,
                    ))
                }
            }
            Err(e) => Err(McpError::internal_error(e.to_string(), None)),
        }
    }

    #[tool(description = "Record a debt ledger")]
    pub async fn record_debt(
        &self,
        #[tool(aggr)] record_debt_model: RecordDebtModel,
    ) -> Result<CallToolResult, McpError> {
        match self.debt_radar_use_case.record(record_debt_model).await {
            Ok(id) => Ok(CallToolResult::success(vec![Content::text(format!(
                "Debt recorded successfully: id: {}",
                id
            ))])),
            Err(e) => Err(McpError::internal_error(e.to_string(), None)),
        }
    }

    #[tool(description = "Record a paid debt")]
    pub async fn record_paid_debt(
        &self,
        #[tool(aggr)] paid_debt_model: PaidDebtModel,
    ) -> Result<CallToolResult, McpError> {
        match self
            .debt_radar_use_case
            .record_paid_debt(paid_debt_model)
            .await
        {
            Ok(id) => Ok(CallToolResult::success(vec![Content::text(format!(
                "Paid debt recorded successfully: id: {}",
                id
            ))])),
            Err(e) => Err(McpError::internal_error(e.to_string(), None)),
        }
    }

    #[tool(description = "See who owes your money and how much")]
    pub async fn view_all_debts(&self) -> Result<CallToolResult, McpError> {
        match self.debt_radar_use_case.view_all().await {
            Ok(results) => {
                if let Ok(res_json) = Content::json(results) {
                    Ok(CallToolResult::success(vec![res_json]))
                } else {
                    Err(McpError::internal_error(
                        "Failed to convert results to JSON".to_string(),
                        None,
                    ))
                }
            }
            Err(e) => Err(McpError::internal_error(e.to_string(), None)),
        }
    }

    #[tool(description = "See how much that person owes you")]
    pub async fn how_that_bro_owe_you(
        &self,
        #[tool(aggr)] who_owes_you_model: WhoOwesMeModel,
    ) -> Result<CallToolResult, McpError> {
        match self
            .debt_radar_use_case
            .view_by_that_bro(&who_owes_you_model.who)
            .await
        {
            Ok(results) => {
                if let Ok(res_json) = Content::json(results) {
                    Ok(CallToolResult::success(vec![res_json]))
                } else {
                    Err(McpError::internal_error(
                        "Failed to convert results to JSON".to_string(),
                        None,
                    ))
                }
            }
            Err(e) => Err(McpError::internal_error(e.to_string(), None)),
        }
    }

    #[tool(description = "Record buy bitcoin ledger")]
    pub async fn record_buy_bitcoin_ledger(
        &self,
        #[tool(aggr)] buy_bitcoin_model: BuyBitcoinModel,
    ) -> Result<CallToolResult, McpError> {
        match self
            .bitcoin_flow_use_case
            .record_buy(buy_bitcoin_model)
            .await
        {
            Ok(id) => Ok(CallToolResult::success(vec![Content::text(format!(
                "Buy bitcoin ledger recorded successfully: id: {}",
                id
            ))])),
            Err(e) => Err(McpError::internal_error(e.to_string(), None)),
        }
    }

    #[tool(description = "Record sell bitcoin ledger")]
    pub async fn record_sell_bitcoin_ledger(
        &self,
        #[tool(aggr)] sell_bitcoin_model: SellBitcoinModel,
    ) -> Result<CallToolResult, McpError> {
        match self
            .bitcoin_flow_use_case
            .record_sell(sell_bitcoin_model)
            .await
        {
            Ok(id) => Ok(CallToolResult::success(vec![Content::text(format!(
                "Buy bitcoin ledger recorded successfully: id: {}",
                id
            ))])),
            Err(e) => Err(McpError::internal_error(e.to_string(), None)),
        }
    }

    #[tool(description = "View all buy bitcoin ledger")]
    pub async fn view_all_buy_bitcoin_ledger(&self) -> Result<CallToolResult, McpError> {
        match self.bitcoin_flow_use_case.view_all_buy().await {
            Ok(results) => {
                if let Ok(res_json) = Content::json(results) {
                    Ok(CallToolResult::success(vec![res_json]))
                } else {
                    Err(McpError::internal_error(
                        "Failed to convert results to JSON".to_string(),
                        None,
                    ))
                }
            }
            Err(e) => Err(McpError::internal_error(e.to_string(), None)),
        }
    }

    #[tool(description = "View all sell bitcoin ledger")]
    pub async fn view_all_sell_bitcoin_ledger(&self) -> Result<CallToolResult, McpError> {
        match self.bitcoin_flow_use_case.view_all_sell().await {
            Ok(results) => {
                if let Ok(res_json) = Content::json(results) {
                    Ok(CallToolResult::success(vec![res_json]))
                } else {
                    Err(McpError::internal_error(
                        "Failed to convert results to JSON".to_string(),
                        None,
                    ))
                }
            }
            Err(e) => Err(McpError::internal_error(e.to_string(), None)),
        }
    }
}

const_string!(Echo = "echo");
#[tool(tool_box)]
impl ServerHandler for MCPHandler {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_prompts()
                .enable_resources()
                .enable_tools()
                .build(),
            server_info: Implementation::from_build_env(),
            instructions: Some("Real personal financial analysis".to_string()),
        }
    }

    async fn list_resources(
        &self,
        _request: Option<PaginatedRequestParam>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, McpError> {
        Ok(ListResourcesResult {
            resources: vec![
                self._create_resource_text("str:////Users/to/some/path/", "cwd"),
                self._create_resource_text("memo://insights", "memo-name"),
            ],
            next_cursor: None,
        })
    }

    async fn read_resource(
        &self,
        ReadResourceRequestParam { uri }: ReadResourceRequestParam,
        _: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResult, McpError> {
        match uri.as_str() {
            "str:////Users/to/some/path/" => {
                let cwd = "/Users/to/some/path/";
                Ok(ReadResourceResult {
                    contents: vec![ResourceContents::text(cwd, uri)],
                })
            }
            "memo://insights" => {
                let memo = "Business Intelligence Memo\n\nAnalysis has revealed 5 key insights ...";
                Ok(ReadResourceResult {
                    contents: vec![ResourceContents::text(memo, uri)],
                })
            }
            _ => Err(McpError::resource_not_found(
                "resource_not_found",
                Some(json!({
                    "uri": uri
                })),
            )),
        }
    }

    async fn list_prompts(
        &self,
        _request: Option<PaginatedRequestParam>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListPromptsResult, McpError> {
        Ok(ListPromptsResult {
            next_cursor: None,
            prompts: vec![Prompt::new(
                "example_prompt",
                Some("This is an example prompt that takes one required argument, message"),
                Some(vec![PromptArgument {
                    name: "message".to_string(),
                    description: Some("A message to put in the prompt".to_string()),
                    required: Some(true),
                }]),
            )],
        })
    }

    async fn get_prompt(
        &self,
        GetPromptRequestParam { name, arguments }: GetPromptRequestParam,
        _: RequestContext<RoleServer>,
    ) -> Result<GetPromptResult, McpError> {
        match name.as_str() {
            "example_prompt" => {
                let message = arguments
                    .and_then(|json| json.get("message")?.as_str().map(|s| s.to_string()))
                    .ok_or_else(|| {
                        McpError::invalid_params("No message provided to example_prompt", None)
                    })?;

                let prompt =
                    format!("This is an example prompt with your message here: '{message}'");
                Ok(GetPromptResult {
                    description: None,
                    messages: vec![PromptMessage {
                        role: PromptMessageRole::User,
                        content: PromptMessageContent::text(prompt),
                    }],
                })
            }
            _ => Err(McpError::invalid_params("prompt not found", None)),
        }
    }

    async fn list_resource_templates(
        &self,
        _request: Option<PaginatedRequestParam>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListResourceTemplatesResult, McpError> {
        Ok(ListResourceTemplatesResult {
            next_cursor: None,
            resource_templates: Vec::new(),
        })
    }
}
