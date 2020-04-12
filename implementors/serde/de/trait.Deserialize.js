(function() {var implementors = {};
implementors["chrono"] = [{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"chrono/naive/struct.NaiveDate.html\" title=\"struct chrono::naive::NaiveDate\">NaiveDate</a>","synthetic":false,"types":["chrono::naive::date::NaiveDate"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"chrono/naive/struct.NaiveTime.html\" title=\"struct chrono::naive::NaiveTime\">NaiveTime</a>","synthetic":false,"types":["chrono::naive::time::NaiveTime"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"chrono/naive/struct.NaiveDateTime.html\" title=\"struct chrono::naive::NaiveDateTime\">NaiveDateTime</a>","synthetic":false,"types":["chrono::naive::datetime::NaiveDateTime"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"chrono/struct.DateTime.html\" title=\"struct chrono::DateTime\">DateTime</a>&lt;<a class=\"struct\" href=\"chrono/offset/struct.FixedOffset.html\" title=\"struct chrono::offset::FixedOffset\">FixedOffset</a>&gt;","synthetic":false,"types":["chrono::datetime::DateTime"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"chrono/struct.DateTime.html\" title=\"struct chrono::DateTime\">DateTime</a>&lt;<a class=\"struct\" href=\"chrono/offset/struct.Utc.html\" title=\"struct chrono::offset::Utc\">Utc</a>&gt;","synthetic":false,"types":["chrono::datetime::DateTime"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"chrono/struct.DateTime.html\" title=\"struct chrono::DateTime\">DateTime</a>&lt;<a class=\"struct\" href=\"chrono/offset/struct.Local.html\" title=\"struct chrono::offset::Local\">Local</a>&gt;","synthetic":false,"types":["chrono::datetime::DateTime"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"enum\" href=\"chrono/enum.Weekday.html\" title=\"enum chrono::Weekday\">Weekday</a>","synthetic":false,"types":["chrono::Weekday"]}];
implementors["jsonwebtoken"] = [{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"enum\" href=\"jsonwebtoken/enum.Algorithm.html\" title=\"enum jsonwebtoken::Algorithm\">Algorithm</a>","synthetic":false,"types":["jsonwebtoken::algorithms::Algorithm"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"jsonwebtoken/struct.Header.html\" title=\"struct jsonwebtoken::Header\">Header</a>","synthetic":false,"types":["jsonwebtoken::header::Header"]}];
implementors["okapi"] = [{"text":"impl&lt;'de, T&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"enum\" href=\"okapi/openapi3/enum.RefOr.html\" title=\"enum okapi::openapi3::RefOr\">RefOr</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt;,&nbsp;</span>","synthetic":false,"types":["okapi::openapi3::RefOr"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"okapi/openapi3/struct.Ref.html\" title=\"struct okapi::openapi3::Ref\">Ref</a>","synthetic":false,"types":["okapi::openapi3::Ref"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"okapi/openapi3/struct.OpenApi.html\" title=\"struct okapi::openapi3::OpenApi\">OpenApi</a>","synthetic":false,"types":["okapi::openapi3::OpenApi"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"okapi/openapi3/struct.Info.html\" title=\"struct okapi::openapi3::Info\">Info</a>","synthetic":false,"types":["okapi::openapi3::Info"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"okapi/openapi3/struct.Contact.html\" title=\"struct okapi::openapi3::Contact\">Contact</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"okapi/openapi3/struct.Contact.html\" title=\"struct okapi::openapi3::Contact\">Contact</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a>,&nbsp;</span>","synthetic":false,"types":["okapi::openapi3::Contact"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"okapi/openapi3/struct.License.html\" title=\"struct okapi::openapi3::License\">License</a>","synthetic":false,"types":["okapi::openapi3::License"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"okapi/openapi3/struct.Server.html\" title=\"struct okapi::openapi3::Server\">Server</a>","synthetic":false,"types":["okapi::openapi3::Server"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"okapi/openapi3/struct.ServerVariable.html\" title=\"struct okapi::openapi3::ServerVariable\">ServerVariable</a>","synthetic":false,"types":["okapi::openapi3::ServerVariable"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"okapi/openapi3/struct.PathItem.html\" title=\"struct okapi::openapi3::PathItem\">PathItem</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"okapi/openapi3/struct.PathItem.html\" title=\"struct okapi::openapi3::PathItem\">PathItem</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a>,&nbsp;</span>","synthetic":false,"types":["okapi::openapi3::PathItem"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"okapi/openapi3/struct.Operation.html\" title=\"struct okapi::openapi3::Operation\">Operation</a>","synthetic":false,"types":["okapi::openapi3::Operation"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"okapi/openapi3/struct.Responses.html\" title=\"struct okapi::openapi3::Responses\">Responses</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"okapi/openapi3/struct.Responses.html\" title=\"struct okapi::openapi3::Responses\">Responses</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a>,&nbsp;</span>","synthetic":false,"types":["okapi::openapi3::Responses"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"okapi/openapi3/struct.Components.html\" title=\"struct okapi::openapi3::Components\">Components</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"okapi/openapi3/struct.Components.html\" title=\"struct okapi::openapi3::Components\">Components</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a>,&nbsp;</span>","synthetic":false,"types":["okapi::openapi3::Components"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"okapi/openapi3/struct.Response.html\" title=\"struct okapi::openapi3::Response\">Response</a>","synthetic":false,"types":["okapi::openapi3::Response"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"okapi/openapi3/struct.Parameter.html\" title=\"struct okapi::openapi3::Parameter\">Parameter</a>","synthetic":false,"types":["okapi::openapi3::Parameter"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"enum\" href=\"okapi/openapi3/enum.ParameterValue.html\" title=\"enum okapi::openapi3::ParameterValue\">ParameterValue</a>","synthetic":false,"types":["okapi::openapi3::ParameterValue"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"enum\" href=\"okapi/openapi3/enum.ParameterStyle.html\" title=\"enum okapi::openapi3::ParameterStyle\">ParameterStyle</a>","synthetic":false,"types":["okapi::openapi3::ParameterStyle"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"okapi/openapi3/struct.Example.html\" title=\"struct okapi::openapi3::Example\">Example</a>","synthetic":false,"types":["okapi::openapi3::Example"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"enum\" href=\"okapi/openapi3/enum.ExampleValue.html\" title=\"enum okapi::openapi3::ExampleValue\">ExampleValue</a>","synthetic":false,"types":["okapi::openapi3::ExampleValue"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"okapi/openapi3/struct.RequestBody.html\" title=\"struct okapi::openapi3::RequestBody\">RequestBody</a>","synthetic":false,"types":["okapi::openapi3::RequestBody"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"okapi/openapi3/struct.Header.html\" title=\"struct okapi::openapi3::Header\">Header</a>","synthetic":false,"types":["okapi::openapi3::Header"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"okapi/openapi3/struct.SecurityScheme.html\" title=\"struct okapi::openapi3::SecurityScheme\">SecurityScheme</a>","synthetic":false,"types":["okapi::openapi3::SecurityScheme"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"enum\" href=\"okapi/openapi3/enum.SecuritySchemeData.html\" title=\"enum okapi::openapi3::SecuritySchemeData\">SecuritySchemeData</a>","synthetic":false,"types":["okapi::openapi3::SecuritySchemeData"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"okapi/openapi3/struct.OAuthFlows.html\" title=\"struct okapi::openapi3::OAuthFlows\">OAuthFlows</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"okapi/openapi3/struct.OAuthFlows.html\" title=\"struct okapi::openapi3::OAuthFlows\">OAuthFlows</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a>,&nbsp;</span>","synthetic":false,"types":["okapi::openapi3::OAuthFlows"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"okapi/openapi3/struct.OAuthFlow.html\" title=\"struct okapi::openapi3::OAuthFlow\">OAuthFlow</a>","synthetic":false,"types":["okapi::openapi3::OAuthFlow"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"okapi/openapi3/struct.Link.html\" title=\"struct okapi::openapi3::Link\">Link</a>","synthetic":false,"types":["okapi::openapi3::Link"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"okapi/openapi3/struct.Callback.html\" title=\"struct okapi::openapi3::Callback\">Callback</a>","synthetic":false,"types":["okapi::openapi3::Callback"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"okapi/openapi3/struct.MediaType.html\" title=\"struct okapi::openapi3::MediaType\">MediaType</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"okapi/openapi3/struct.MediaType.html\" title=\"struct okapi::openapi3::MediaType\">MediaType</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a>,&nbsp;</span>","synthetic":false,"types":["okapi::openapi3::MediaType"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"okapi/openapi3/struct.Tag.html\" title=\"struct okapi::openapi3::Tag\">Tag</a>","synthetic":false,"types":["okapi::openapi3::Tag"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"okapi/openapi3/struct.ExternalDocs.html\" title=\"struct okapi::openapi3::ExternalDocs\">ExternalDocs</a>","synthetic":false,"types":["okapi::openapi3::ExternalDocs"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"okapi/openapi3/struct.Encoding.html\" title=\"struct okapi::openapi3::Encoding\">Encoding</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"okapi/openapi3/struct.Encoding.html\" title=\"struct okapi::openapi3::Encoding\">Encoding</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a>,&nbsp;</span>","synthetic":false,"types":["okapi::openapi3::Encoding"]}];
implementors["potatosync_rust"] = [{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"potatosync_rust/note/model/struct.Note.html\" title=\"struct potatosync_rust::note::model::Note\">Note</a>","synthetic":false,"types":["potatosync_rust::note::model::Note"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"potatosync_rust/note/model/struct.NewNote.html\" title=\"struct potatosync_rust::note::model::NewNote\">NewNote</a>","synthetic":false,"types":["potatosync_rust::note::model::NewNote"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"potatosync_rust/note/model/struct.SavingNote.html\" title=\"struct potatosync_rust::note::model::SavingNote\">SavingNote</a>","synthetic":false,"types":["potatosync_rust::note::model::SavingNote"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"potatosync_rust/note/model/struct.NoteId.html\" title=\"struct potatosync_rust::note::model::NoteId\">NoteId</a>","synthetic":false,"types":["potatosync_rust::note::model::NoteId"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"potatosync_rust/note/model/struct.NoteLastUpdated.html\" title=\"struct potatosync_rust::note::model::NoteLastUpdated\">NoteLastUpdated</a>","synthetic":false,"types":["potatosync_rust::note::model::NoteLastUpdated"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"potatosync_rust/note/model/struct.NoteResponse.html\" title=\"struct potatosync_rust::note::model::NoteResponse\">NoteResponse</a>","synthetic":false,"types":["potatosync_rust::note::model::NoteResponse"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"potatosync_rust/account/model/struct.Account.html\" title=\"struct potatosync_rust::account::model::Account\">Account</a>","synthetic":false,"types":["potatosync_rust::account::model::Account"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"potatosync_rust/account/model/struct.NewDBAccount.html\" title=\"struct potatosync_rust::account::model::NewDBAccount\">NewDBAccount</a>","synthetic":false,"types":["potatosync_rust::account::model::NewDBAccount"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"potatosync_rust/account/model/struct.LoginCredentials.html\" title=\"struct potatosync_rust::account::model::LoginCredentials\">LoginCredentials</a>","synthetic":false,"types":["potatosync_rust::account::model::LoginCredentials"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"potatosync_rust/account/model/struct.Password.html\" title=\"struct potatosync_rust::account::model::Password\">Password</a>","synthetic":false,"types":["potatosync_rust::account::model::Password"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"potatosync_rust/account/model/struct.Username.html\" title=\"struct potatosync_rust::account::model::Username\">Username</a>","synthetic":false,"types":["potatosync_rust::account::model::Username"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"potatosync_rust/account/model/struct.Image.html\" title=\"struct potatosync_rust::account::model::Image\">Image</a>","synthetic":false,"types":["potatosync_rust::account::model::Image"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"potatosync_rust/account/model/struct.NewAccount.html\" title=\"struct potatosync_rust::account::model::NewAccount\">NewAccount</a>","synthetic":false,"types":["potatosync_rust::account::model::NewAccount"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"potatosync_rust/account/model/struct.TokenResponse.html\" title=\"struct potatosync_rust::account::model::TokenResponse\">TokenResponse</a>","synthetic":false,"types":["potatosync_rust::account::model::TokenResponse"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"potatosync_rust/account/model/struct.TokenAccount.html\" title=\"struct potatosync_rust::account::model::TokenAccount\">TokenAccount</a>","synthetic":false,"types":["potatosync_rust::account::model::TokenAccount"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"potatosync_rust/account/model/struct.InfoResponse.html\" title=\"struct potatosync_rust::account::model::InfoResponse\">InfoResponse</a>","synthetic":false,"types":["potatosync_rust::account::model::InfoResponse"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"potatosync_rust/account/token/struct.Token.html\" title=\"struct potatosync_rust::account::token::Token\">Token</a>","synthetic":false,"types":["potatosync_rust::account::token::Token"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"potatosync_rust/account/token/struct.RefreshTokenDb.html\" title=\"struct potatosync_rust::account::token::RefreshTokenDb\">RefreshTokenDb</a>","synthetic":false,"types":["potatosync_rust::account::token::RefreshTokenDb"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"potatosync_rust/account/token/struct.RefreshToken.html\" title=\"struct potatosync_rust::account::token::RefreshToken\">RefreshToken</a>","synthetic":false,"types":["potatosync_rust::account::token::RefreshToken"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"potatosync_rust/account/token/struct.RefreshTokenJson.html\" title=\"struct potatosync_rust::account::token::RefreshTokenJson\">RefreshTokenJson</a>","synthetic":false,"types":["potatosync_rust::account::token::RefreshTokenJson"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"potatosync_rust/account/token/struct.RefreshResponse.html\" title=\"struct potatosync_rust::account::token::RefreshResponse\">RefreshResponse</a>","synthetic":false,"types":["potatosync_rust::account::token::RefreshResponse"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"potatosync_rust/account/email/struct.VerificationToken.html\" title=\"struct potatosync_rust::account::email::VerificationToken\">VerificationToken</a>","synthetic":false,"types":["potatosync_rust::account::email::VerificationToken"]}];
implementors["rocket_contrib"] = [{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"rocket_contrib/json/struct.JsonValue.html\" title=\"struct rocket_contrib::json::JsonValue\">JsonValue</a>","synthetic":false,"types":["rocket_contrib::json::JsonValue"]}];
implementors["rocket_failure"] = [{"text":"impl&lt;'de, T&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"enum\" href=\"rocket_failure/errors/enum.StrictApiResponse.html\" title=\"enum rocket_failure::errors::StrictApiResponse\">StrictApiResponse</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt;,&nbsp;</span>","synthetic":false,"types":["rocket_failure::errors::StrictApiResponse"]},{"text":"impl&lt;'de, T&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"enum\" href=\"rocket_failure/enum.StrictApiResponse.html\" title=\"enum rocket_failure::StrictApiResponse\">StrictApiResponse</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt;,&nbsp;</span>","synthetic":false,"types":["rocket_failure::StrictApiResponse"]}];
implementors["rocket_okapi"] = [{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"rocket_okapi/swagger_ui/struct.SwaggerUIConfig.html\" title=\"struct rocket_okapi::swagger_ui::SwaggerUIConfig\">SwaggerUIConfig</a>","synthetic":false,"types":["rocket_okapi::swagger_ui::SwaggerUIConfig"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"rocket_okapi/swagger_ui/struct.UrlObject.html\" title=\"struct rocket_okapi::swagger_ui::UrlObject\">UrlObject</a>","synthetic":false,"types":["rocket_okapi::swagger_ui::UrlObject"]}];
implementors["schemars"] = [{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"enum\" href=\"schemars/schema/enum.Schema.html\" title=\"enum schemars::schema::Schema\">Schema</a>","synthetic":false,"types":["schemars::schema::Schema"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"schemars/schema/struct.RootSchema.html\" title=\"struct schemars::schema::RootSchema\">RootSchema</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"schemars/schema/struct.RootSchema.html\" title=\"struct schemars::schema::RootSchema\">RootSchema</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a>,&nbsp;</span>","synthetic":false,"types":["schemars::schema::RootSchema"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"schemars/schema/struct.SchemaObject.html\" title=\"struct schemars::schema::SchemaObject\">SchemaObject</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"schemars/schema/struct.SchemaObject.html\" title=\"struct schemars::schema::SchemaObject\">SchemaObject</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a>,&nbsp;</span>","synthetic":false,"types":["schemars::schema::SchemaObject"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"schemars/schema/struct.Metadata.html\" title=\"struct schemars::schema::Metadata\">Metadata</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"schemars/schema/struct.Metadata.html\" title=\"struct schemars::schema::Metadata\">Metadata</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a>,&nbsp;</span>","synthetic":false,"types":["schemars::schema::Metadata"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"schemars/schema/struct.SubschemaValidation.html\" title=\"struct schemars::schema::SubschemaValidation\">SubschemaValidation</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"schemars/schema/struct.SubschemaValidation.html\" title=\"struct schemars::schema::SubschemaValidation\">SubschemaValidation</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a>,&nbsp;</span>","synthetic":false,"types":["schemars::schema::SubschemaValidation"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"schemars/schema/struct.NumberValidation.html\" title=\"struct schemars::schema::NumberValidation\">NumberValidation</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"schemars/schema/struct.NumberValidation.html\" title=\"struct schemars::schema::NumberValidation\">NumberValidation</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a>,&nbsp;</span>","synthetic":false,"types":["schemars::schema::NumberValidation"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"schemars/schema/struct.StringValidation.html\" title=\"struct schemars::schema::StringValidation\">StringValidation</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"schemars/schema/struct.StringValidation.html\" title=\"struct schemars::schema::StringValidation\">StringValidation</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a>,&nbsp;</span>","synthetic":false,"types":["schemars::schema::StringValidation"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"schemars/schema/struct.ArrayValidation.html\" title=\"struct schemars::schema::ArrayValidation\">ArrayValidation</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"schemars/schema/struct.ArrayValidation.html\" title=\"struct schemars::schema::ArrayValidation\">ArrayValidation</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a>,&nbsp;</span>","synthetic":false,"types":["schemars::schema::ArrayValidation"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"schemars/schema/struct.ObjectValidation.html\" title=\"struct schemars::schema::ObjectValidation\">ObjectValidation</a> <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"schemars/schema/struct.ObjectValidation.html\" title=\"struct schemars::schema::ObjectValidation\">ObjectValidation</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a>,&nbsp;</span>","synthetic":false,"types":["schemars::schema::ObjectValidation"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"enum\" href=\"schemars/schema/enum.InstanceType.html\" title=\"enum schemars::schema::InstanceType\">InstanceType</a>","synthetic":false,"types":["schemars::schema::InstanceType"]},{"text":"impl&lt;'de, T&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"enum\" href=\"schemars/schema/enum.SingleOrVec.html\" title=\"enum schemars::schema::SingleOrVec\">SingleOrVec</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt;,&nbsp;</span>","synthetic":false,"types":["schemars::schema::SingleOrVec"]}];
implementors["serde_json"] = [{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"serde_json/map/struct.Map.html\" title=\"struct serde_json::map::Map\">Map</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>, <a class=\"enum\" href=\"serde_json/value/enum.Value.html\" title=\"enum serde_json::value::Value\">Value</a>&gt;","synthetic":false,"types":["serde_json::map::Map"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"enum\" href=\"serde_json/value/enum.Value.html\" title=\"enum serde_json::value::Value\">Value</a>","synthetic":false,"types":["serde_json::value::Value"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"serde_json/struct.Number.html\" title=\"struct serde_json::Number\">Number</a>","synthetic":false,"types":["serde_json::number::Number"]}];
implementors["toml"] = [{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"enum\" href=\"toml/value/enum.Value.html\" title=\"enum toml::value::Value\">Value</a>","synthetic":false,"types":["toml::value::Value"]},{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"toml/value/struct.Datetime.html\" title=\"struct toml::value::Datetime\">Datetime</a>","synthetic":false,"types":["toml::datetime::Datetime"]}];
implementors["validator"] = [{"text":"impl&lt;'de&gt; <a class=\"trait\" href=\"serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"validator/struct.ValidationError.html\" title=\"struct validator::ValidationError\">ValidationError</a>","synthetic":false,"types":["validator::types::ValidationError"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()