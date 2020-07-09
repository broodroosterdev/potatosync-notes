initSidebarItems({"enum":[["ValidationErrorsKind",""],["Validator","Contains all the validators that can be used"]],"fn":[["validate_contains","Validates whether the value contains the needle The value needs to implement the Contains trait, which is implement on String, str and Hashmap by default."],["validate_email","Validates whether the given string is an email based on Django `EmailValidator` and HTML5 specs"],["validate_ip","Validates whether the given string is an IP"],["validate_ip_v4","Validates whether the given string is an IP V4"],["validate_ip_v6","Validates whether the given string is an IP V6"],["validate_length","Validates the length of the value given. If the validator has `equal` set, it will ignore any `min` and `max` value."],["validate_must_match","Validates that the 2 given fields match. Both fields are optionals"],["validate_range","Validates that a number is in the given range"],["validate_required","Validates whether the given Option is Some"],["validate_url","Validates whether the string given is a url"]],"struct":[["ValidationError",""],["ValidationErrors",""]],"trait":[["Contains","Trait to implement if one wants to make the `contains` validator work for more types"],["HasLen","Trait to implement if one wants to make the `length` validator work for more types"],["Validate","The trait that `validator_derive` implements"]]});