// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

//! "core" DataFusion functions

mod nullif;
mod nvl;

// create UDFs
make_udf_function!(nullif::NullIfFunc, NULLIF, nullif);
make_udf_function!(nvl::NVLFunc, NVL, nvl);

// Export the functions out of this package, both as expr_fn as well as a list of functions
export_functions!(
    (nullif, arg_1 arg_2, "returns NULL if value1 equals value2; otherwise it returns value1. This can be used to perform the inverse operation of the COALESCE expression."),
    (nvl, arg_1 arg_2, "returns value2 if value1 is NULL; otherwise it returns value1")
);

