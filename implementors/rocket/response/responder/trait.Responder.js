(function() {var implementors = {};
implementors["potatosync_notes"] = [{"text":"impl&lt;'r&gt; <a class=\"trait\" href=\"rocket/response/responder/trait.Responder.html\" title=\"trait rocket::response::responder::Responder\">Responder</a>&lt;'r&gt; for <a class=\"struct\" href=\"potatosync_notes/note/controller/struct.NoteResponse.html\" title=\"struct potatosync_notes::note::controller::NoteResponse\">NoteResponse</a>","synthetic":false,"types":["potatosync_notes::note::controller::NoteResponse"]},{"text":"impl&lt;'r&gt; <a class=\"trait\" href=\"rocket/response/responder/trait.Responder.html\" title=\"trait rocket::response::responder::Responder\">Responder</a>&lt;'r&gt; for <a class=\"struct\" href=\"potatosync_notes/note/controller/struct.DeletedResponse.html\" title=\"struct potatosync_notes::note::controller::DeletedResponse\">DeletedResponse</a>","synthetic":false,"types":["potatosync_notes::note::controller::DeletedResponse"]},{"text":"impl&lt;'r&gt; <a class=\"trait\" href=\"rocket/response/responder/trait.Responder.html\" title=\"trait rocket::response::responder::Responder\">Responder</a>&lt;'r&gt; for <a class=\"struct\" href=\"potatosync_notes/responders/struct.ApiResponse.html\" title=\"struct potatosync_notes::responders::ApiResponse\">ApiResponse</a>","synthetic":false,"types":["potatosync_notes::responders::ApiResponse"]},{"text":"impl&lt;'r&gt; <a class=\"trait\" href=\"rocket/response/responder/trait.Responder.html\" title=\"trait rocket::response::responder::Responder\">Responder</a>&lt;'r&gt; for <a class=\"struct\" href=\"potatosync_notes/responders/struct.ApiError.html\" title=\"struct potatosync_notes::responders::ApiError\">ApiError</a>","synthetic":false,"types":["potatosync_notes::responders::ApiError"]},{"text":"impl&lt;'r&gt; <a class=\"trait\" href=\"rocket/response/responder/trait.Responder.html\" title=\"trait rocket::response::responder::Responder\">Responder</a>&lt;'r&gt; for <a class=\"struct\" href=\"potatosync_notes/responders/struct.ApiSuccess.html\" title=\"struct potatosync_notes::responders::ApiSuccess\">ApiSuccess</a>","synthetic":false,"types":["potatosync_notes::responders::ApiSuccess"]}];
implementors["rocket_contrib"] = [{"text":"impl&lt;'a, T:&nbsp;<a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a>&gt; <a class=\"trait\" href=\"rocket/response/responder/trait.Responder.html\" title=\"trait rocket::response::responder::Responder\">Responder</a>&lt;'a&gt; for <a class=\"struct\" href=\"rocket_contrib/json/struct.Json.html\" title=\"struct rocket_contrib::json::Json\">Json</a>&lt;T&gt;","synthetic":false,"types":["rocket_contrib::json::Json"]},{"text":"impl&lt;'a&gt; <a class=\"trait\" href=\"rocket/response/responder/trait.Responder.html\" title=\"trait rocket::response::responder::Responder\">Responder</a>&lt;'a&gt; for <a class=\"struct\" href=\"rocket_contrib/json/struct.JsonValue.html\" title=\"struct rocket_contrib::json::JsonValue\">JsonValue</a>","synthetic":false,"types":["rocket_contrib::json::JsonValue"]}];
implementors["rocket_failure"] = [{"text":"impl&lt;'r, T&gt; <a class=\"trait\" href=\"rocket/response/responder/trait.Responder.html\" title=\"trait rocket::response::responder::Responder\">Responder</a>&lt;'r&gt; for <a class=\"enum\" href=\"rocket_failure/errors/enum.ApiResponse.html\" title=\"enum rocket_failure::errors::ApiResponse\">ApiResponse</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a>,&nbsp;</span>","synthetic":false,"types":["rocket_failure::errors::ApiResponse"]},{"text":"impl&lt;'r&gt; <a class=\"trait\" href=\"rocket/response/responder/trait.Responder.html\" title=\"trait rocket::response::responder::Responder\">Responder</a>&lt;'r&gt; for <a class=\"struct\" href=\"rocket_failure/errors/struct.WebError.html\" title=\"struct rocket_failure::errors::WebError\">WebError</a>","synthetic":false,"types":["rocket_failure::errors::WebError"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()