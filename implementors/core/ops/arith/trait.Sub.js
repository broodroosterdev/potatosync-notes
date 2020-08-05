(function() {var implementors = {};
implementors["chrono"] = [{"text":"impl Sub&lt;FixedOffset&gt; for NaiveTime","synthetic":false,"types":[]},{"text":"impl Sub&lt;FixedOffset&gt; for NaiveDateTime","synthetic":false,"types":[]},{"text":"impl&lt;Tz:&nbsp;TimeZone&gt; Sub&lt;FixedOffset&gt; for DateTime&lt;Tz&gt;","synthetic":false,"types":[]},{"text":"impl Sub&lt;Duration&gt; for NaiveDate","synthetic":false,"types":[]},{"text":"impl Sub&lt;NaiveDate&gt; for NaiveDate","synthetic":false,"types":[]},{"text":"impl Sub&lt;Duration&gt; for NaiveDateTime","synthetic":false,"types":[]},{"text":"impl Sub&lt;NaiveDateTime&gt; for NaiveDateTime","synthetic":false,"types":[]},{"text":"impl Sub&lt;Duration&gt; for NaiveTime","synthetic":false,"types":[]},{"text":"impl Sub&lt;NaiveTime&gt; for NaiveTime","synthetic":false,"types":[]},{"text":"impl&lt;Tz:&nbsp;TimeZone&gt; Sub&lt;Duration&gt; for Date&lt;Tz&gt;","synthetic":false,"types":[]},{"text":"impl&lt;Tz:&nbsp;TimeZone&gt; Sub&lt;Date&lt;Tz&gt;&gt; for Date&lt;Tz&gt;","synthetic":false,"types":[]},{"text":"impl&lt;Tz:&nbsp;TimeZone&gt; Sub&lt;Duration&gt; for DateTime&lt;Tz&gt;","synthetic":false,"types":[]},{"text":"impl&lt;Tz:&nbsp;TimeZone&gt; Sub&lt;DateTime&lt;Tz&gt;&gt; for DateTime&lt;Tz&gt;","synthetic":false,"types":[]}];
implementors["devise_core"] = [{"text":"impl Sub&lt;GenericSupport&gt; for GenericSupport","synthetic":false,"types":[]},{"text":"impl Sub&lt;DataSupport&gt; for DataSupport","synthetic":false,"types":[]}];
implementors["diesel"] = [{"text":"impl&lt;Rhs&gt; Sub&lt;Rhs&gt; for now <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Rhs: AsExpression&lt;&lt;&lt;now as Expression&gt;::SqlType as Sub&gt;::Rhs&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;ST, T, __Rhs&gt; Sub&lt;__Rhs&gt; for SqlLiteral&lt;ST, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Self: Expression,<br>&nbsp;&nbsp;&nbsp;&nbsp;Self::SqlType: Sub,<br>&nbsp;&nbsp;&nbsp;&nbsp;__Rhs: AsExpression&lt;&lt;Self::SqlType as Sub&gt;::Rhs&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl Sub&lt;PgMoney&gt; for PgMoney","synthetic":false,"types":[]}];
implementors["hashbrown"] = [{"text":"impl&lt;T, S, '_, '_&gt; Sub&lt;&amp;'_ HashSet&lt;T, S&gt;&gt; for &amp;'_ HashSet&lt;T, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Eq + Hash + Clone,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: BuildHasher + Default,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["indexmap"] = [{"text":"impl&lt;'a, 'b, T, S1, S2&gt; Sub&lt;&amp;'b IndexSet&lt;T, S2&gt;&gt; for &amp;'a IndexSet&lt;T, S1&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Eq + Hash + Clone,<br>&nbsp;&nbsp;&nbsp;&nbsp;S1: BuildHasher + Default,<br>&nbsp;&nbsp;&nbsp;&nbsp;S2: BuildHasher,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["inotify"] = [{"text":"impl Sub&lt;EventMask&gt; for EventMask","synthetic":false,"types":[]},{"text":"impl Sub&lt;WatchMask&gt; for WatchMask","synthetic":false,"types":[]}];
implementors["mio"] = [{"text":"impl Sub&lt;PollOpt&gt; for PollOpt","synthetic":false,"types":[]},{"text":"impl&lt;T:&nbsp;Into&lt;Ready&gt;&gt; Sub&lt;T&gt; for Ready","synthetic":false,"types":[]},{"text":"impl Sub&lt;UnixReady&gt; for UnixReady","synthetic":false,"types":[]}];
implementors["notify"] = [{"text":"impl Sub&lt;Op&gt; for Op","synthetic":false,"types":[]}];
implementors["num_bigint"] = [{"text":"impl&lt;'a, 'b&gt; Sub&lt;&amp;'b BigInt&gt; for &amp;'a BigInt","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;BigInt&gt; for &amp;'a BigInt","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;&amp;'a BigInt&gt; for BigInt","synthetic":false,"types":[]},{"text":"impl Sub&lt;BigInt&gt; for BigInt","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;&amp;'a u8&gt; for BigInt","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;BigInt&gt; for &amp;'a u8","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;u8&gt; for &amp;'a BigInt","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;&amp;'a BigInt&gt; for u8","synthetic":false,"types":[]},{"text":"impl&lt;'a, 'b&gt; Sub&lt;&amp;'b u8&gt; for &amp;'a BigInt","synthetic":false,"types":[]},{"text":"impl&lt;'a, 'b&gt; Sub&lt;&amp;'a BigInt&gt; for &amp;'b u8","synthetic":false,"types":[]},{"text":"impl Sub&lt;u8&gt; for BigInt","synthetic":false,"types":[]},{"text":"impl Sub&lt;BigInt&gt; for u8","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;&amp;'a u16&gt; for BigInt","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;BigInt&gt; for &amp;'a u16","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;u16&gt; for &amp;'a BigInt","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;&amp;'a BigInt&gt; for u16","synthetic":false,"types":[]},{"text":"impl&lt;'a, 'b&gt; Sub&lt;&amp;'b u16&gt; for &amp;'a BigInt","synthetic":false,"types":[]},{"text":"impl&lt;'a, 'b&gt; Sub&lt;&amp;'a BigInt&gt; for &amp;'b u16","synthetic":false,"types":[]},{"text":"impl Sub&lt;u16&gt; for BigInt","synthetic":false,"types":[]},{"text":"impl Sub&lt;BigInt&gt; for u16","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;&amp;'a usize&gt; for BigInt","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;BigInt&gt; for &amp;'a usize","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;usize&gt; for &amp;'a BigInt","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;&amp;'a BigInt&gt; for usize","synthetic":false,"types":[]},{"text":"impl&lt;'a, 'b&gt; Sub&lt;&amp;'b usize&gt; for &amp;'a BigInt","synthetic":false,"types":[]},{"text":"impl&lt;'a, 'b&gt; Sub&lt;&amp;'a BigInt&gt; for &amp;'b usize","synthetic":false,"types":[]},{"text":"impl Sub&lt;usize&gt; for BigInt","synthetic":false,"types":[]},{"text":"impl Sub&lt;BigInt&gt; for usize","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;&amp;'a i8&gt; for BigInt","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;BigInt&gt; for &amp;'a i8","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;i8&gt; for &amp;'a BigInt","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;&amp;'a BigInt&gt; for i8","synthetic":false,"types":[]},{"text":"impl&lt;'a, 'b&gt; Sub&lt;&amp;'b i8&gt; for &amp;'a BigInt","synthetic":false,"types":[]},{"text":"impl&lt;'a, 'b&gt; Sub&lt;&amp;'a BigInt&gt; for &amp;'b i8","synthetic":false,"types":[]},{"text":"impl Sub&lt;i8&gt; for BigInt","synthetic":false,"types":[]},{"text":"impl Sub&lt;BigInt&gt; for i8","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;&amp;'a i16&gt; for BigInt","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;BigInt&gt; for &amp;'a i16","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;i16&gt; for &amp;'a BigInt","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;&amp;'a BigInt&gt; for i16","synthetic":false,"types":[]},{"text":"impl&lt;'a, 'b&gt; Sub&lt;&amp;'b i16&gt; for &amp;'a BigInt","synthetic":false,"types":[]},{"text":"impl&lt;'a, 'b&gt; Sub&lt;&amp;'a BigInt&gt; for &amp;'b i16","synthetic":false,"types":[]},{"text":"impl Sub&lt;i16&gt; for BigInt","synthetic":false,"types":[]},{"text":"impl Sub&lt;BigInt&gt; for i16","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;&amp;'a isize&gt; for BigInt","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;BigInt&gt; for &amp;'a isize","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;isize&gt; for &amp;'a BigInt","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;&amp;'a BigInt&gt; for isize","synthetic":false,"types":[]},{"text":"impl&lt;'a, 'b&gt; Sub&lt;&amp;'b isize&gt; for &amp;'a BigInt","synthetic":false,"types":[]},{"text":"impl&lt;'a, 'b&gt; Sub&lt;&amp;'a BigInt&gt; for &amp;'b isize","synthetic":false,"types":[]},{"text":"impl Sub&lt;isize&gt; for BigInt","synthetic":false,"types":[]},{"text":"impl Sub&lt;BigInt&gt; for isize","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;&amp;'a u32&gt; for BigInt","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;BigInt&gt; for &amp;'a u32","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;u32&gt; for &amp;'a BigInt","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;&amp;'a BigInt&gt; for u32","synthetic":false,"types":[]},{"text":"impl&lt;'a, 'b&gt; Sub&lt;&amp;'b u32&gt; for &amp;'a BigInt","synthetic":false,"types":[]},{"text":"impl&lt;'a, 'b&gt; Sub&lt;&amp;'a BigInt&gt; for &amp;'b u32","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;&amp;'a u64&gt; for BigInt","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;BigInt&gt; for &amp;'a u64","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;u64&gt; for &amp;'a BigInt","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;&amp;'a BigInt&gt; for u64","synthetic":false,"types":[]},{"text":"impl&lt;'a, 'b&gt; Sub&lt;&amp;'b u64&gt; for &amp;'a BigInt","synthetic":false,"types":[]},{"text":"impl&lt;'a, 'b&gt; Sub&lt;&amp;'a BigInt&gt; for &amp;'b u64","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;&amp;'a u128&gt; for BigInt","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;BigInt&gt; for &amp;'a u128","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;u128&gt; for &amp;'a BigInt","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;&amp;'a BigInt&gt; for u128","synthetic":false,"types":[]},{"text":"impl&lt;'a, 'b&gt; Sub&lt;&amp;'b u128&gt; for &amp;'a BigInt","synthetic":false,"types":[]},{"text":"impl&lt;'a, 'b&gt; Sub&lt;&amp;'a BigInt&gt; for &amp;'b u128","synthetic":false,"types":[]},{"text":"impl Sub&lt;u32&gt; for BigInt","synthetic":false,"types":[]},{"text":"impl Sub&lt;BigInt&gt; for u32","synthetic":false,"types":[]},{"text":"impl Sub&lt;BigInt&gt; for u64","synthetic":false,"types":[]},{"text":"impl Sub&lt;BigInt&gt; for u128","synthetic":false,"types":[]},{"text":"impl Sub&lt;u64&gt; for BigInt","synthetic":false,"types":[]},{"text":"impl Sub&lt;u128&gt; for BigInt","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;&amp;'a i32&gt; for BigInt","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;BigInt&gt; for &amp;'a i32","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;i32&gt; for &amp;'a BigInt","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;&amp;'a BigInt&gt; for i32","synthetic":false,"types":[]},{"text":"impl&lt;'a, 'b&gt; Sub&lt;&amp;'b i32&gt; for &amp;'a BigInt","synthetic":false,"types":[]},{"text":"impl&lt;'a, 'b&gt; Sub&lt;&amp;'a BigInt&gt; for &amp;'b i32","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;&amp;'a i64&gt; for BigInt","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;BigInt&gt; for &amp;'a i64","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;i64&gt; for &amp;'a BigInt","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;&amp;'a BigInt&gt; for i64","synthetic":false,"types":[]},{"text":"impl&lt;'a, 'b&gt; Sub&lt;&amp;'b i64&gt; for &amp;'a BigInt","synthetic":false,"types":[]},{"text":"impl&lt;'a, 'b&gt; Sub&lt;&amp;'a BigInt&gt; for &amp;'b i64","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;&amp;'a i128&gt; for BigInt","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;BigInt&gt; for &amp;'a i128","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;i128&gt; for &amp;'a BigInt","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;&amp;'a BigInt&gt; for i128","synthetic":false,"types":[]},{"text":"impl&lt;'a, 'b&gt; Sub&lt;&amp;'b i128&gt; for &amp;'a BigInt","synthetic":false,"types":[]},{"text":"impl&lt;'a, 'b&gt; Sub&lt;&amp;'a BigInt&gt; for &amp;'b i128","synthetic":false,"types":[]},{"text":"impl Sub&lt;i32&gt; for BigInt","synthetic":false,"types":[]},{"text":"impl Sub&lt;BigInt&gt; for i32","synthetic":false,"types":[]},{"text":"impl Sub&lt;i64&gt; for BigInt","synthetic":false,"types":[]},{"text":"impl Sub&lt;BigInt&gt; for i64","synthetic":false,"types":[]},{"text":"impl Sub&lt;i128&gt; for BigInt","synthetic":false,"types":[]},{"text":"impl Sub&lt;BigInt&gt; for i128","synthetic":false,"types":[]},{"text":"impl Sub&lt;BigUint&gt; for BigUint","synthetic":false,"types":[]},{"text":"impl&lt;'a, 'b&gt; Sub&lt;&amp;'b BigUint&gt; for &amp;'a BigUint","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;&amp;'a BigUint&gt; for BigUint","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;BigUint&gt; for &amp;'a BigUint","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;&amp;'a u8&gt; for BigUint","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;BigUint&gt; for &amp;'a u8","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;u8&gt; for &amp;'a BigUint","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;&amp;'a BigUint&gt; for u8","synthetic":false,"types":[]},{"text":"impl&lt;'a, 'b&gt; Sub&lt;&amp;'b u8&gt; for &amp;'a BigUint","synthetic":false,"types":[]},{"text":"impl&lt;'a, 'b&gt; Sub&lt;&amp;'a BigUint&gt; for &amp;'b u8","synthetic":false,"types":[]},{"text":"impl Sub&lt;u8&gt; for BigUint","synthetic":false,"types":[]},{"text":"impl Sub&lt;BigUint&gt; for u8","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;&amp;'a u16&gt; for BigUint","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;BigUint&gt; for &amp;'a u16","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;u16&gt; for &amp;'a BigUint","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;&amp;'a BigUint&gt; for u16","synthetic":false,"types":[]},{"text":"impl&lt;'a, 'b&gt; Sub&lt;&amp;'b u16&gt; for &amp;'a BigUint","synthetic":false,"types":[]},{"text":"impl&lt;'a, 'b&gt; Sub&lt;&amp;'a BigUint&gt; for &amp;'b u16","synthetic":false,"types":[]},{"text":"impl Sub&lt;u16&gt; for BigUint","synthetic":false,"types":[]},{"text":"impl Sub&lt;BigUint&gt; for u16","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;&amp;'a usize&gt; for BigUint","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;BigUint&gt; for &amp;'a usize","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;usize&gt; for &amp;'a BigUint","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;&amp;'a BigUint&gt; for usize","synthetic":false,"types":[]},{"text":"impl&lt;'a, 'b&gt; Sub&lt;&amp;'b usize&gt; for &amp;'a BigUint","synthetic":false,"types":[]},{"text":"impl&lt;'a, 'b&gt; Sub&lt;&amp;'a BigUint&gt; for &amp;'b usize","synthetic":false,"types":[]},{"text":"impl Sub&lt;usize&gt; for BigUint","synthetic":false,"types":[]},{"text":"impl Sub&lt;BigUint&gt; for usize","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;&amp;'a u32&gt; for BigUint","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;BigUint&gt; for &amp;'a u32","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;u32&gt; for &amp;'a BigUint","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;&amp;'a BigUint&gt; for u32","synthetic":false,"types":[]},{"text":"impl&lt;'a, 'b&gt; Sub&lt;&amp;'b u32&gt; for &amp;'a BigUint","synthetic":false,"types":[]},{"text":"impl&lt;'a, 'b&gt; Sub&lt;&amp;'a BigUint&gt; for &amp;'b u32","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;&amp;'a u64&gt; for BigUint","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;BigUint&gt; for &amp;'a u64","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;u64&gt; for &amp;'a BigUint","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;&amp;'a BigUint&gt; for u64","synthetic":false,"types":[]},{"text":"impl&lt;'a, 'b&gt; Sub&lt;&amp;'b u64&gt; for &amp;'a BigUint","synthetic":false,"types":[]},{"text":"impl&lt;'a, 'b&gt; Sub&lt;&amp;'a BigUint&gt; for &amp;'b u64","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;&amp;'a u128&gt; for BigUint","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;BigUint&gt; for &amp;'a u128","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;u128&gt; for &amp;'a BigUint","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Sub&lt;&amp;'a BigUint&gt; for u128","synthetic":false,"types":[]},{"text":"impl&lt;'a, 'b&gt; Sub&lt;&amp;'b u128&gt; for &amp;'a BigUint","synthetic":false,"types":[]},{"text":"impl&lt;'a, 'b&gt; Sub&lt;&amp;'a BigUint&gt; for &amp;'b u128","synthetic":false,"types":[]},{"text":"impl Sub&lt;u32&gt; for BigUint","synthetic":false,"types":[]},{"text":"impl Sub&lt;BigUint&gt; for u32","synthetic":false,"types":[]},{"text":"impl Sub&lt;u64&gt; for BigUint","synthetic":false,"types":[]},{"text":"impl Sub&lt;BigUint&gt; for u64","synthetic":false,"types":[]},{"text":"impl Sub&lt;u128&gt; for BigUint","synthetic":false,"types":[]},{"text":"impl Sub&lt;BigUint&gt; for u128","synthetic":false,"types":[]}];
implementors["openssl"] = [{"text":"impl&lt;'a, 'b&gt; Sub&lt;&amp;'b BigNumRef&gt; for &amp;'a BigNumRef","synthetic":false,"types":[]},{"text":"impl&lt;'a, 'b&gt; Sub&lt;&amp;'b BigNum&gt; for &amp;'a BigNumRef","synthetic":false,"types":[]},{"text":"impl&lt;'a, 'b&gt; Sub&lt;&amp;'b BigNumRef&gt; for &amp;'a BigNum","synthetic":false,"types":[]},{"text":"impl&lt;'a, 'b&gt; Sub&lt;&amp;'b BigNum&gt; for &amp;'a BigNum","synthetic":false,"types":[]},{"text":"impl Sub&lt;CMSOptions&gt; for CMSOptions","synthetic":false,"types":[]},{"text":"impl Sub&lt;OcspFlag&gt; for OcspFlag","synthetic":false,"types":[]},{"text":"impl Sub&lt;Pkcs7Flags&gt; for Pkcs7Flags","synthetic":false,"types":[]},{"text":"impl Sub&lt;SslOptions&gt; for SslOptions","synthetic":false,"types":[]},{"text":"impl Sub&lt;SslMode&gt; for SslMode","synthetic":false,"types":[]},{"text":"impl Sub&lt;SslVerifyMode&gt; for SslVerifyMode","synthetic":false,"types":[]},{"text":"impl Sub&lt;SslSessionCacheMode&gt; for SslSessionCacheMode","synthetic":false,"types":[]},{"text":"impl Sub&lt;ExtensionContext&gt; for ExtensionContext","synthetic":false,"types":[]},{"text":"impl Sub&lt;ShutdownState&gt; for ShutdownState","synthetic":false,"types":[]},{"text":"impl Sub&lt;X509CheckFlags&gt; for X509CheckFlags","synthetic":false,"types":[]}];
implementors["potatosync_notes"] = [{"text":"impl&lt;Rhs&gt; Sub&lt;Rhs&gt; for creation_date <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Rhs: AsExpression&lt;&lt;&lt;creation_date as Expression&gt;::SqlType as Sub&gt;::Rhs&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;Rhs&gt; Sub&lt;Rhs&gt; for last_modify_date <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Rhs: AsExpression&lt;&lt;&lt;last_modify_date as Expression&gt;::SqlType as Sub&gt;::Rhs&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;Rhs&gt; Sub&lt;Rhs&gt; for color <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Rhs: AsExpression&lt;&lt;&lt;color as Expression&gt;::SqlType as Sub&gt;::Rhs&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;Rhs&gt; Sub&lt;Rhs&gt; for last_modify_date <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Rhs: AsExpression&lt;&lt;&lt;last_modify_date as Expression&gt;::SqlType as Sub&gt;::Rhs&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;Rhs&gt; Sub&lt;Rhs&gt; for color <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Rhs: AsExpression&lt;&lt;&lt;color as Expression&gt;::SqlType as Sub&gt;::Rhs&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;Rhs&gt; Sub&lt;Rhs&gt; for last_modify_date <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Rhs: AsExpression&lt;&lt;&lt;last_modify_date as Expression&gt;::SqlType as Sub&gt;::Rhs&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["time"] = [{"text":"impl Sub&lt;Duration&gt; for Duration","synthetic":false,"types":[]},{"text":"impl Sub&lt;Duration&gt; for Timespec","synthetic":false,"types":[]},{"text":"impl Sub&lt;Timespec&gt; for Timespec","synthetic":false,"types":[]},{"text":"impl Sub&lt;SteadyTime&gt; for SteadyTime","synthetic":false,"types":[]},{"text":"impl Sub&lt;Duration&gt; for SteadyTime","synthetic":false,"types":[]},{"text":"impl Sub&lt;Duration&gt; for Tm","synthetic":false,"types":[]},{"text":"impl Sub&lt;Tm&gt; for Tm","synthetic":false,"types":[]}];
implementors["tokio"] = [{"text":"impl Sub&lt;Instant&gt; for Instant","synthetic":false,"types":[]},{"text":"impl Sub&lt;Duration&gt; for Instant","synthetic":false,"types":[]}];
implementors["typenum"] = [{"text":"impl Sub&lt;Z0&gt; for Z0","synthetic":false,"types":[]},{"text":"impl&lt;U:&nbsp;Unsigned + NonZero&gt; Sub&lt;PInt&lt;U&gt;&gt; for Z0","synthetic":false,"types":[]},{"text":"impl&lt;U:&nbsp;Unsigned + NonZero&gt; Sub&lt;NInt&lt;U&gt;&gt; for Z0","synthetic":false,"types":[]},{"text":"impl&lt;U:&nbsp;Unsigned + NonZero&gt; Sub&lt;Z0&gt; for PInt&lt;U&gt;","synthetic":false,"types":[]},{"text":"impl&lt;U:&nbsp;Unsigned + NonZero&gt; Sub&lt;Z0&gt; for NInt&lt;U&gt;","synthetic":false,"types":[]},{"text":"impl&lt;Ul:&nbsp;Unsigned + NonZero, Ur:&nbsp;Unsigned + NonZero&gt; Sub&lt;NInt&lt;Ur&gt;&gt; for PInt&lt;Ul&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Ul: Add&lt;Ur&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;Ul as Add&lt;Ur&gt;&gt;::Output: Unsigned + NonZero,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;Ul:&nbsp;Unsigned + NonZero, Ur:&nbsp;Unsigned + NonZero&gt; Sub&lt;PInt&lt;Ur&gt;&gt; for NInt&lt;Ul&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Ul: Add&lt;Ur&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;Ul as Add&lt;Ur&gt;&gt;::Output: Unsigned + NonZero,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;Ul:&nbsp;Unsigned + NonZero, Ur:&nbsp;Unsigned + NonZero&gt; Sub&lt;PInt&lt;Ur&gt;&gt; for PInt&lt;Ul&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Ul: Cmp&lt;Ur&gt; + PrivateIntegerAdd&lt;&lt;Ul as Cmp&lt;Ur&gt;&gt;::Output, Ur&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;Ul:&nbsp;Unsigned + NonZero, Ur:&nbsp;Unsigned + NonZero&gt; Sub&lt;NInt&lt;Ur&gt;&gt; for NInt&lt;Ul&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Ur: Cmp&lt;Ul&gt; + PrivateIntegerAdd&lt;&lt;Ur as Cmp&lt;Ul&gt;&gt;::Output, Ul&gt;,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl Sub&lt;B0&gt; for UTerm","synthetic":false,"types":[]},{"text":"impl&lt;U:&nbsp;Unsigned, B:&nbsp;Bit&gt; Sub&lt;B0&gt; for UInt&lt;U, B&gt;","synthetic":false,"types":[]},{"text":"impl&lt;U:&nbsp;Unsigned, B:&nbsp;Bit&gt; Sub&lt;B1&gt; for UInt&lt;UInt&lt;U, B&gt;, B1&gt;","synthetic":false,"types":[]},{"text":"impl Sub&lt;B1&gt; for UInt&lt;UTerm, B1&gt;","synthetic":false,"types":[]},{"text":"impl&lt;U:&nbsp;Unsigned&gt; Sub&lt;B1&gt; for UInt&lt;U, B0&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;U: Sub&lt;B1&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;Sub1&lt;U&gt;: Unsigned,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl Sub&lt;UTerm&gt; for UTerm","synthetic":false,"types":[]},{"text":"impl&lt;Ul:&nbsp;Unsigned, Bl:&nbsp;Bit, Ur:&nbsp;Unsigned&gt; Sub&lt;Ur&gt; for UInt&lt;Ul, Bl&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;UInt&lt;Ul, Bl&gt;: PrivateSub&lt;Ur&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;PrivateSubOut&lt;UInt&lt;Ul, Bl&gt;, Ur&gt;: Trim,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl Sub&lt;ATerm&gt; for ATerm","synthetic":false,"types":[]},{"text":"impl&lt;Vl, Al, Vr, Ar&gt; Sub&lt;TArr&lt;Vr, Ar&gt;&gt; for TArr&lt;Vl, Al&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Vl: Sub&lt;Vr&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;Al: Sub&lt;Ar&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()