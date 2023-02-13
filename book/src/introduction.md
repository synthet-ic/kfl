<table>
  <tr>
    <td>
      <pre lang="kdl"><code>node0</code></pre>
    </td>
    <td>
      <pre lang="kdl"><code>node1 1 "hoge"</code></pre>
    </td>
    <td>
      <pre lang="kdl"><code>node2 a=1 b="hoge"</code></pre>
    </td>
  </tr>
  <tr>
    <td colspan=3><i>structs</i></td> 
  </tr>
  <tr>
    <td rowspan=2>
      <pre>#[derive(Decode)]
struct Node0;</pre>
    </td>
    <td>
      <pre>#[derive(Decode)]
struct Node1(
    #[kfl(argument)] i32,
    #[kfl(argument)] String
);</pre>
    </td>
    <td>
      <pre>#[derive(Decode)]
struct Node2(
    #[kfl(property(name = "a"))] i32,
    #[kfl(property(name = "b"))] String
);</pre>
    </td>
  </tr>
  <tr>
    <td>
      <pre>#[derive(Decode)]
struct Node1 {
    #[kfl(argument)] a: i32,
    #[kfl(argument)] b: String
}</pre>
    </td>
    <td>
      <pre>#[derive(Decode)]
struct Node2 {
    #[kfl(property)] a: i32,
    #[kfl(property)] b: String
}</pre>
    </td>
  </tr>
  <tr>
    <td colspan=3><i>enums</i></td> 
  </tr>
  <tr>
    <td colspan=3>
      <pre>                          #[derive(Decode)]
                            enum Node {
                                Node0,
                                Node1(
                                    #[kfl(argument)] i32,
                                    #[kfl(argument)] String
                                ),
                                Node2(
                                    #[kfl(property(name = "a"))] i32,
                                    #[kfl(property(name = "b"))] String
                                )
                            }</pre>
    </td>
  </tr>
  <tr>
    <td colspan=3>
      <pre>                          #[derive(Decode)]
                            enum Node {
                                Node0,
                                Node1 {
                                    #[kfl(argument)] a: i32,
                                    #[kfl(argument)] b: String
                                },
                                Node2 {
                                    #[kfl(property)] a: i32,
                                    #[kfl(property)] b: String
                                }
                            }</pre>
    </td>
  </tr>
</table>
