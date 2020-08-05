(function() {var implementors = {};
implementors["chrono"] = [{"text":"impl Serialize for NaiveDate","synthetic":false,"types":[]},{"text":"impl Serialize for NaiveDateTime","synthetic":false,"types":[]},{"text":"impl Serialize for NaiveTime","synthetic":false,"types":[]},{"text":"impl&lt;Tz:&nbsp;TimeZone&gt; Serialize for DateTime&lt;Tz&gt;","synthetic":false,"types":[]},{"text":"impl Serialize for Weekday","synthetic":false,"types":[]}];
implementors["jsonwebtoken"] = [{"text":"impl Serialize for Algorithm","synthetic":false,"types":[]},{"text":"impl Serialize for Header","synthetic":false,"types":[]}];
implementors["potatosync_notes"] = [{"text":"impl Serialize for Note","synthetic":false,"types":[]},{"text":"impl Serialize for SavingNote","synthetic":false,"types":[]},{"text":"impl Serialize for PatchingNote","synthetic":false,"types":[]},{"text":"impl Serialize for NoteId","synthetic":false,"types":[]},{"text":"impl Serialize for NoteLastUpdated","synthetic":false,"types":[]},{"text":"impl Serialize for NoteResponse","synthetic":false,"types":[]},{"text":"impl Serialize for DeletedResponse","synthetic":false,"types":[]},{"text":"impl Serialize for Setting","synthetic":false,"types":[]},{"text":"impl Serialize for Tag","synthetic":false,"types":[]},{"text":"impl Serialize for SavingTag","synthetic":false,"types":[]},{"text":"impl Serialize for PatchingTag","synthetic":false,"types":[]},{"text":"impl Serialize for ApiError","synthetic":false,"types":[]},{"text":"impl Serialize for ApiSuccess","synthetic":false,"types":[]},{"text":"impl Serialize for StatusResponse","synthetic":false,"types":[]}];
implementors["rocket_contrib"] = [{"text":"impl Serialize for JsonValue","synthetic":false,"types":[]}];
implementors["rocket_failure"] = [{"text":"impl&lt;T&gt; Serialize for StrictApiResponse&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Serialize,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["rocket_failure_errors"] = [{"text":"impl&lt;T&gt; Serialize for StrictApiResponse&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Serialize,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["serde_json"] = [{"text":"impl Serialize for Map&lt;String, Value&gt;","synthetic":false,"types":[]},{"text":"impl Serialize for Value","synthetic":false,"types":[]},{"text":"impl Serialize for Number","synthetic":false,"types":[]}];
implementors["toml"] = [{"text":"impl Serialize for Value","synthetic":false,"types":[]},{"text":"impl Serialize for Datetime","synthetic":false,"types":[]}];
implementors["uuid"] = [{"text":"impl Serialize for Uuid","synthetic":false,"types":[]}];
implementors["validator"] = [{"text":"impl Serialize for ValidationError","synthetic":false,"types":[]},{"text":"impl Serialize for ValidationErrorsKind","synthetic":false,"types":[]},{"text":"impl Serialize for ValidationErrors","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()