// =========================================================================================================================
/*
 * Copyright (C) 2024 Tan Jun Kiat
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
*/
// =========================================================================================================================
use std::path::PathBuf;
use std::sync::Arc;
use opcua::server::prelude::*;
use opcua::sync::Mutex;

fn main() {
    // Create an OPC UA server with sample configuration and default node set
    let mut server = Server::new(ServerConfig::load(&PathBuf::from("assets/opcua_server.conf")).unwrap());

    let ns = {
        let address_space = server.address_space();
        let mut address_space = address_space.write();
        address_space
            .register_namespace("urn:simple-server")
            .unwrap()
    };

    // Add some variables of our own
    add_example_variables(&mut server, ns);

    // Run the server. This does not ordinarily exit so you must Ctrl+C to terminate
    server.run();
}

/// Creates some sample variables, and some push / pull examples that update them
fn add_example_variables(server: &mut Server, ns: u16) {
    // These will be the node ids of the new variables
    let v1_node = NodeId::new(ns, "v1");
    let v2_node = NodeId::new(ns, "v2");
    let v3_node = NodeId::new(ns, "v3");
    let v4_node = NodeId::new(ns, "v4");
    

    let address_space = server.address_space();
    // The address space is guarded so obtain a lock to change it
    {
        let mut address_space = address_space.write();

        // Create a sample folder under objects folder
        let sample_folder_id = address_space
            .add_folder("Sample", "Sample", &NodeId::objects_folder_id())
            .unwrap();

        // Add some variables to our sample folder. Values will be overwritten by the timer
        let _ = address_space.add_variables(
            vec![
                Variable::new(&v1_node, "v1", "v1", 0_i32),
                Variable::new(&v2_node, "v2", "v2", false),
                Variable::new(&v3_node, "v3", "v3", UAString::from("")),
                Variable::new(&v4_node, "v4", "v4", 0f64),
            ],
            &sample_folder_id,
        );

        let node = address_space.find_variable_mut(v4_node.clone()).unwrap();
        node.set_user_access_level(UserAccessLevel::CURRENT_WRITE | UserAccessLevel::CURRENT_READ);
    }

    // OPC UA for Rust allows you to push or pull values from a variable so here are examples
    // of each method.

    // 1) Pull. This code will add getters to v3 & v4 that returns their values by calling
    //    function.
    {
        let address_space = server.address_space();
        let mut address_space = address_space.write();
        if let Some(ref mut v) = address_space.find_variable_mut(v3_node.clone()) {
            // Hello world's counter will increment with each get - slower interval == slower increment
            let mut counter = 0;
            let getter = AttrFnGetter::new(
                move |_, _, _, _, _, _| -> Result<Option<DataValue>, StatusCode> {
                    counter += 1;
                    Ok(Some(DataValue::new_now(UAString::from(format!(
                        "Hello World times {}",
                        counter
                    )))))
                },
            );
            v.set_value_getter(Arc::new(Mutex::new(getter)));
        }
    }

    // 2) Push. This code will use a timer to set the values on variable v1 & v2 on an interval.
    //    Note you can use any kind of timer callback that you like for this. The library
    //    contains a simple add_polling_action for your convenience.
    {
        // Store a counter and a flag in a tuple
        let data = Arc::new(Mutex::new((0, true)));
        server.add_polling_action(300, move || {
            let mut data = data.lock();
            data.0 += 1;
            data.1 = !data.1;
            let mut address_space = address_space.write();
            let now = DateTime::now();
            let _ = address_space.set_variable_value(v1_node.clone(), data.0, &now, &now);
            let _ = address_space.set_variable_value(v2_node.clone(), data.1, &now, &now);
        });
    }
}