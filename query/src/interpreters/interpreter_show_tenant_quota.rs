// Copyright 2022 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::sync::Arc;

use common_datablocks::DataBlock;
use common_datavalues::prelude::*;
use common_exception::Result;
use common_planners::ShowTenantQuotaPlan;
use common_streams::DataBlockStream;
use common_streams::SendableDataBlockStream;

use super::Interpreter;
use crate::interpreters::InterpreterPtr;
use crate::sessions::QueryContext;

pub struct ShowTenantQuotaInterpreter {
    ctx: Arc<QueryContext>,
}

impl ShowTenantQuotaInterpreter {
    pub fn try_create(
        ctx: Arc<QueryContext>,
        _plan: ShowTenantQuotaPlan,
    ) -> Result<InterpreterPtr> {
        Ok(Arc::new(ShowTenantQuotaInterpreter { ctx }))
    }
}

#[async_trait::async_trait]
impl Interpreter for ShowTenantQuotaInterpreter {
    fn name(&self) -> &str {
        "ShowTenantQuotaInterpreter"
    }

    async fn execute(
        &self,
        _input_stream: Option<SendableDataBlockStream>,
    ) -> Result<SendableDataBlockStream> {
        let tenant = self.ctx.get_tenant();
        let quota_api = self
            .ctx
            .get_user_manager()
            .get_tenant_quota_api_client(&tenant)?;
        let quota = quota_api.get_quota(None).await?.data;
        let schema = DataSchemaRefExt::create(vec![
            DataField::new("max_databases", u32::to_data_type()),
            DataField::new("max_tables_per_database", u32::to_data_type()),
            DataField::new("max_stages", u32::to_data_type()),
            DataField::new("max_files_per_stage", u32::to_data_type()),
        ]);
        let block = DataBlock::create(schema.clone(), vec![
            Series::from_data(vec![quota.max_databases]),
            Series::from_data(vec![quota.max_tables_per_database]),
            Series::from_data(vec![quota.max_stages]),
            Series::from_data(vec![quota.max_files_per_stage]),
        ]);
        Ok(Box::pin(DataBlockStream::create(schema, None, vec![block])))
    }
}
