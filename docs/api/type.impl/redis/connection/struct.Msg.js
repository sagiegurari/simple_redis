(function() {var type_impls = {
"simple_redis":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Msg\" class=\"impl\"><a href=\"#impl-Msg\" class=\"anchor\">§</a><h3 class=\"code-header\">impl Msg</h3></section></summary><div class=\"docblock\"><p>This holds the data that comes from listening to a pubsub\nconnection.  It only contains actual message data.</p>\n</div><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from_value\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">from_value</a>(value: &amp;Value) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;Msg&gt;</h4></section></summary><div class=\"docblock\"><p>Tries to convert provided [<code>Value</code>] into [<code>Msg</code>].</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.get_channel\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">get_channel</a>&lt;T&gt;(&amp;self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;T, RedisError&gt;<div class=\"where\">where\n    T: FromRedisValue,</div></h4></section></summary><div class=\"docblock\"><p>Returns the channel this message came on.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.get_channel_name\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">get_channel_name</a>(&amp;self) -&gt; &amp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a></h4></section></summary><div class=\"docblock\"><p>Convenience method to get a string version of the channel.  Unless\nyour channel contains non utf-8 bytes you can always use this\nmethod.  If the channel is not a valid string (which really should\nnot happen) then the return value is <code>&quot;?&quot;</code>.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.get_payload\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">get_payload</a>&lt;T&gt;(&amp;self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;T, RedisError&gt;<div class=\"where\">where\n    T: FromRedisValue,</div></h4></section></summary><div class=\"docblock\"><p>Returns the message’s payload in a specific format.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.get_payload_bytes\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">get_payload_bytes</a>(&amp;self) -&gt; &amp;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>] <a href=\"#\" class=\"tooltip\" data-notable-ty=\"&amp;[u8]\">ⓘ</a></h4></section></summary><div class=\"docblock\"><p>Returns the bytes that are the message’s payload.  This can be used\nas an alternative to the <code>get_payload</code> function if you are interested\nin the raw bytes in it.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.from_pattern\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">from_pattern</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a></h4></section></summary><div class=\"docblock\"><p>Returns true if the message was constructed from a pattern\nsubscription.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.get_pattern\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">get_pattern</a>&lt;T&gt;(&amp;self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;T, RedisError&gt;<div class=\"where\">where\n    T: FromRedisValue,</div></h4></section></summary><div class=\"docblock\"><p>If the message was constructed from a message pattern this can be\nused to find out which one.  It’s recommended to match against\nan <code>Option&lt;String&gt;</code> so that you do not need to use <code>from_pattern</code>\nto figure out if a pattern was set.</p>\n</div></details></div></details>",0,"simple_redis::types::Message"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-Msg\" class=\"impl\"><a href=\"#impl-Debug-for-Msg\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for Msg</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html\" title=\"struct core::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","simple_redis::types::Message"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()