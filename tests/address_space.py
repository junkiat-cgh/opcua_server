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
from datetime import datetime
import asyncio
from asyncua import Client

async def collect_all_under(node, collect_maximum=500000):
    all_nodes = []
    unsearched = [node]

    while unsearched and len(all_nodes) < collect_maximum:
        child = unsearched.pop()
        all_nodes.append(child)
        children = await child.get_children()
        unsearched.extend(children)

    return all_nodes

async def main():
    url = 'opc.tcp://localhost:4840'
    nodeid = "i=85"
    async with Client(url=url) as client:
        node = client.get_node(nodeid)

        start = datetime.now()
        # pass a number here as 2nd arg to change the max count collected
        nodes_found = await collect_all_under(node)
        duration = datetime.now() - start
        count = len(nodes_found)
        rate = count / duration.total_seconds()

        print("we found {} nodes in {} ({} per second)".format(
            count, duration, rate))
        print("First 10 are:\n" + "\n".join(str(x) for x in nodes_found[:10]))

        return nodes_found

if __name__ == '__main__':
    asyncio.run(main())