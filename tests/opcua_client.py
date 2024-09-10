'''
=========================================================================================================================
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
=========================================================================================================================
'''
import asyncio
from asyncua import Client
url = "opc.tcp://localhost:4840"
namespace = "urn:simple-server"

async def main():

    print(f"Connecting to {url} ...")
    async with Client(url=url) as client:
        nsidx = await client.get_namespace_index(namespace)
        print(f"Namespace Index for '{namespace}': {nsidx}")

        v1_value = await client.get_node('ns=2;s=v1').read_value()
        v2_value = await client.get_node('ns=2;s=v2').read_value()
        v3_value = await client.get_node('ns=2;s=v3').read_value()
        v4_value = await client.get_node('ns=2;s=v4').read_value()
        print(f"v1 value: {v1_value}")
        print(f"v2 value: {v2_value}")
        print(f"v3 value: {v3_value}")
        print(f"v4 value: {v4_value}")

        access_level = await client.get_node('ns=2;s=v4').get_access_level()
        print(f"v4 access level: {access_level}")

        await client.get_node('ns=2;s=v4').write_value(50.0)
        print(f"Writing to v4: 50.0")
        v4_value = await client.get_node('ns=2;s=v4').read_value()
        print(f"v4 new value: {v4_value}")


if __name__ == "__main__":
    asyncio.run(main())
