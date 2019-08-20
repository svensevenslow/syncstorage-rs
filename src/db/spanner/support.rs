use std::collections::HashMap;

use protobuf::well_known_types::Value;

use super::models::Result;
use crate::db::DbError;

#[cfg(google_grpc)]
type ParamValue = protobuf::well_known_types::Value;
#[cfg(not(google_grpc))]
type ParamValue = String;

#[cfg(google_grpc)]
type ParamType = googleapis_raw::spanner::v1::type_pb::Type;
#[cfg(not(google_grpc))]
type ParamType = google_spanner1::Type;

#[cfg(google_grpc)]
type ExecuteSqlRequest = googleapis_raw::spanner::v1::ExecuteSqlRequest;
#[cfg(not(google_grpc))]
type ExecuteSqlRequest = google_spanner1::ExecuteSqlRequest;

#[cfg(google_grpc)]
type ExecuteResult = ::grpcio::Result<googleapis_raw::spanner::v1::result_set::ResultSet>;
#[cfg(not(google_grpc))]
type ExecuteResult = google_spanner1::Result<(hyper::client::Response, google_spanner1::ResultSet)>;

#[cfg(google_grpc)]
type ResultSet = googleapis_raw::spanner::v1::result_set::ResultSet;
#[cfg(not(google_grpc))]
type ResultSet = google_spanner1::ResultSet;

// XXX: or Into<protobuf Value>?
#[cfg(google_grpc)]
pub fn as_value(string_value: String) -> protobuf::well_known_types::Value {
    let mut value = Value::new();
    value.set_string_value(string_value);
    value
}

#[cfg(not(google_grpc))]
pub fn as_value(string_value: String) -> String {
    string_value
}

#[allow(dead_code)]
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum SpannerType {
    TypeCodeUnspecified,
    Bool,
    Int64,
    Float64,
    Timestamp,
    Date,
    String,
    Bytes,
    Array,
    Struct,
}

#[cfg(google_grpc)]
impl Into<googleapis_raw::spanner::v1::type_pb::Type> for SpannerType {
    fn into(self) -> googleapis_raw::spanner::v1::type_pb::Type {
        let mut t = googleapis_raw::spanner::v1::type_pb::Type::new();
        use googleapis_raw::spanner::v1::type_pb::TypeCode;
        let code = match self {
            SpannerType::TypeCodeUnspecified => TypeCode::TYPE_CODE_UNSPECIFIED,
            SpannerType::Bool => TypeCode::BOOL,
            SpannerType::Int64 => TypeCode::INT64,
            SpannerType::Float64 => TypeCode::FLOAT64,
            SpannerType::Timestamp => TypeCode::TIMESTAMP,
            SpannerType::Date => TypeCode::DATE,
            SpannerType::String => TypeCode::STRING,
            SpannerType::Bytes => TypeCode::BYTES,
            SpannerType::Array => TypeCode::ARRAY,
            SpannerType::Struct => TypeCode::STRUCT,
        };
        t.set_code(code);
        t
    }
}

impl Into<google_spanner1::Type> for SpannerType {
    fn into(self) -> google_spanner1::Type {
        let code = match self {
            SpannerType::TypeCodeUnspecified => "TYPE_CODE_UNSPECIFIED",
            SpannerType::Bool => "BOOL",
            SpannerType::Int64 => "INT64",
            SpannerType::Float64 => "FLOAT64",
            SpannerType::Timestamp => "TIMESTAMP",
            SpannerType::Date => "DATE",
            SpannerType::String => "STRING",
            SpannerType::Bytes => "BYTES",
            SpannerType::Array => "ARRAY",
            SpannerType::Struct => "STRUCT",
        };
        google_spanner1::Type {
            code: Some(code.to_owned()),
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct ExecuteSqlRequestBuilder {
    execute_sql: ExecuteSqlRequest,
    params: Option<HashMap<String, ParamValue>>,
    param_types: Option<HashMap<String, ParamType>>,
}

impl ExecuteSqlRequestBuilder {
    pub fn new(execute_sql: ExecuteSqlRequest) -> Self {
        ExecuteSqlRequestBuilder {
            execute_sql,
            ..Default::default()
        }
    }

    pub fn params(mut self, params: HashMap<String, ParamValue>) -> Self {
        self.params = Some(params);
        self
    }

    pub fn param_types(mut self, param_types: HashMap<String, ParamType>) -> Self {
        self.param_types = Some(param_types);
        self
    }

    #[cfg(not(google_grpc))]
    //pub fn execute(self, spanner: &super::models::Conn) -> ExecuteResult {
    pub fn execute(self, spanner: &super::models::Conn) -> Result<SyncResultSet> {
        /*
        let session = spanner
            .session
            .name
            .as_ref()
            .ok_or_else(|| DbError::internal("No spanner session"))?;
        */
        let session = spanner.session.name.as_ref().unwrap();
        let mut request = self.execute_sql;
        request.params = self.params;
        request.param_types = self.param_types;
        /*
        spanner
            .hub
            .projects()
            .instances_databases_sessions_execute_sql(request, session)
            .doit()
        */
        let (_, result) = spanner
            .hub
            .projects()
            .instances_databases_sessions_execute_sql(request, session)
            .doit()?;
        Ok(SyncResultSet { result })
    }
}

pub struct SyncResultSet {
    result: ResultSet,
}

/*
#[cfg(google_grpc)]
impl SyncResultSet {
    pub fn one(&mut self) -> Result<Vec<Value>> {
        if let Some(result) = self.one_or_none()? {
            Ok(result)
        } else {
            Err(DbError::internal("No rows matched the given query."))?
        }
    }

    pub fn one_or_none(&mut self) -> Result<Option<Vec<Value>>> {}
}

#[cfg(not(google_grpc))]
*/
impl SyncResultSet {
    pub fn one(&mut self) -> Result<Vec<Value>> {
        if let Some(result) = self.one_or_none()? {
            Ok(result)
        } else {
            Err(DbError::internal("No rows matched the given query."))?
        }
    }

    pub fn one_or_none(&mut self) -> Result<Option<Vec<Value>>> {
        let result = self.next();
        if result.is_none() {
            Ok(None)
        } else {
            if self.next().is_some() {
                Err(DbError::internal("Execpted one result; got more."))?
            } else {
                Ok(result)
            }
        }
    }
}

#[cfg(not(google_grpc))]
impl Iterator for SyncResultSet {
    type Item = Vec<Value>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(rows) = self.result.rows.as_mut() {
            rows.pop().map(|row| {
                row.into_iter()
                    .map(|s| {
                        let mut value = Value::new();
                        value.set_string_value(s);
                        value
                    })
                    .collect()
            })
        } else {
            None
        }
    }
}
