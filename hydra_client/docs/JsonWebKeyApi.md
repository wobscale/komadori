# \JsonWebKeyApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_json_web_key_set**](JsonWebKeyApi.md#create_json_web_key_set) | **Post** /keys/{set} | Generate a new JSON Web Key
[**delete_json_web_key**](JsonWebKeyApi.md#delete_json_web_key) | **Delete** /keys/{set}/{kid} | Delete a JSON Web Key
[**delete_json_web_key_set**](JsonWebKeyApi.md#delete_json_web_key_set) | **Delete** /keys/{set} | Delete a JSON Web Key Set
[**get_json_web_key**](JsonWebKeyApi.md#get_json_web_key) | **Get** /keys/{set}/{kid} | Retrieve a JSON Web Key
[**get_json_web_key_set**](JsonWebKeyApi.md#get_json_web_key_set) | **Get** /keys/{set} | Retrieve a JSON Web Key Set
[**update_json_web_key**](JsonWebKeyApi.md#update_json_web_key) | **Put** /keys/{set}/{kid} | Update a JSON Web Key
[**update_json_web_key_set**](JsonWebKeyApi.md#update_json_web_key_set) | **Put** /keys/{set} | Update a JSON Web Key Set


# **create_json_web_key_set**
> ::models::JsonWebKeySet create_json_web_key_set(ctx, set, optional)
Generate a new JSON Web Key

This endpoint is capable of generating JSON Web Key Sets for you. There a different strategies available, such as symmetric cryptographic keys (HS256, HS512) and asymetric cryptographic keys (RS256, ECDSA). If the specified JSON Web Key Set does not exist, it will be created.  A JSON Web Key (JWK) is a JavaScript Object Notation (JSON) data structure that represents a cryptographic key. A JWK Set is a JSON data structure that represents a set of JWKs. A JSON Web Key is identified by its set and key id. ORY Hydra uses this functionality to store cryptographic keys used for TLS and JSON Web Tokens (such as OpenID Connect ID tokens), and allows storing user-defined keys as well.  The subject making the request needs to be assigned to a policy containing:  ``` { \"resources\": [\"rn:hydra:keys:<set>:<kid>\"], \"actions\": [\"create\"], \"effect\": \"allow\" } ```

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **set** | **String**| The set | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **set** | **String**| The set | 
 **json_web_key_set_generator_request** | [**JsonWebKeySetGeneratorRequest**](JsonWebKeySetGeneratorRequest.md)|  | 

### Return type

[**::models::JsonWebKeySet**](jsonWebKeySet.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_json_web_key**
> delete_json_web_key(ctx, kid, set)
Delete a JSON Web Key

Use this endpoint to delete a single JSON Web Key.  A JSON Web Key (JWK) is a JavaScript Object Notation (JSON) data structure that represents a cryptographic key. A JWK Set is a JSON data structure that represents a set of JWKs. A JSON Web Key is identified by its set and key id. ORY Hydra uses this functionality to store cryptographic keys used for TLS and JSON Web Tokens (such as OpenID Connect ID tokens), and allows storing user-defined keys as well.  The subject making the request needs to be assigned to a policy containing:  ``` { \"resources\": [\"rn:hydra:keys:<set>:<kid>\"], \"actions\": [\"delete\"], \"effect\": \"allow\" } ```

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **kid** | **String**| The kid of the desired key | 
  **set** | **String**| The set | 

### Return type

 (empty response body)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_json_web_key_set**
> delete_json_web_key_set(ctx, set)
Delete a JSON Web Key Set

Use this endpoint to delete a complete JSON Web Key Set and all the keys in that set.  A JSON Web Key (JWK) is a JavaScript Object Notation (JSON) data structure that represents a cryptographic key. A JWK Set is a JSON data structure that represents a set of JWKs. A JSON Web Key is identified by its set and key id. ORY Hydra uses this functionality to store cryptographic keys used for TLS and JSON Web Tokens (such as OpenID Connect ID tokens), and allows storing user-defined keys as well.  The subject making the request needs to be assigned to a policy containing:  ``` { \"resources\": [\"rn:hydra:keys:<set>\"], \"actions\": [\"delete\"], \"effect\": \"allow\" } ```

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **set** | **String**| The set | 

### Return type

 (empty response body)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_json_web_key**
> ::models::JsonWebKeySet get_json_web_key(ctx, kid, set)
Retrieve a JSON Web Key

This endpoint can be used to retrieve JWKs stored in ORY Hydra.  A JSON Web Key (JWK) is a JavaScript Object Notation (JSON) data structure that represents a cryptographic key. A JWK Set is a JSON data structure that represents a set of JWKs. A JSON Web Key is identified by its set and key id. ORY Hydra uses this functionality to store cryptographic keys used for TLS and JSON Web Tokens (such as OpenID Connect ID tokens), and allows storing user-defined keys as well.  The subject making the request needs to be assigned to a policy containing:  ``` { \"resources\": [\"rn:hydra:keys:<set>:<kid>\"], \"actions\": [\"get\"], \"effect\": \"allow\" } ```

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **kid** | **String**| The kid of the desired key | 
  **set** | **String**| The set | 

### Return type

[**::models::JsonWebKeySet**](jsonWebKeySet.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_json_web_key_set**
> ::models::JsonWebKeySet get_json_web_key_set(ctx, set)
Retrieve a JSON Web Key Set

This endpoint can be used to retrieve JWK Sets stored in ORY Hydra.  A JSON Web Key (JWK) is a JavaScript Object Notation (JSON) data structure that represents a cryptographic key. A JWK Set is a JSON data structure that represents a set of JWKs. A JSON Web Key is identified by its set and key id. ORY Hydra uses this functionality to store cryptographic keys used for TLS and JSON Web Tokens (such as OpenID Connect ID tokens), and allows storing user-defined keys as well.  The subject making the request needs to be assigned to a policy containing:  ``` { \"resources\": [\"rn:hydra:keys:<set>:<kid>\"], \"actions\": [\"get\"], \"effect\": \"allow\" } ```

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **set** | **String**| The set | 

### Return type

[**::models::JsonWebKeySet**](jsonWebKeySet.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_json_web_key**
> ::models::JsonWebKey update_json_web_key(ctx, kid, set, optional)
Update a JSON Web Key

Use this method if you do not want to let Hydra generate the JWKs for you, but instead save your own.  A JSON Web Key (JWK) is a JavaScript Object Notation (JSON) data structure that represents a cryptographic key. A JWK Set is a JSON data structure that represents a set of JWKs. A JSON Web Key is identified by its set and key id. ORY Hydra uses this functionality to store cryptographic keys used for TLS and JSON Web Tokens (such as OpenID Connect ID tokens), and allows storing user-defined keys as well.  The subject making the request needs to be assigned to a policy containing:  ``` { \"resources\": [\"rn:hydra:keys:<set>:<kid>\"], \"actions\": [\"update\"], \"effect\": \"allow\" } ```

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **kid** | **String**| The kid of the desired key | 
  **set** | **String**| The set | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **kid** | **String**| The kid of the desired key | 
 **set** | **String**| The set | 
 **json_web_key** | [**JsonWebKey**](JsonWebKey.md)|  | 

### Return type

[**::models::JsonWebKey**](jsonWebKey.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_json_web_key_set**
> ::models::JsonWebKeySet update_json_web_key_set(ctx, set, optional)
Update a JSON Web Key Set

Use this method if you do not want to let Hydra generate the JWKs for you, but instead save your own.  A JSON Web Key (JWK) is a JavaScript Object Notation (JSON) data structure that represents a cryptographic key. A JWK Set is a JSON data structure that represents a set of JWKs. A JSON Web Key is identified by its set and key id. ORY Hydra uses this functionality to store cryptographic keys used for TLS and JSON Web Tokens (such as OpenID Connect ID tokens), and allows storing user-defined keys as well.  The subject making the request needs to be assigned to a policy containing:  ``` { \"resources\": [\"rn:hydra:keys:<set>\"], \"actions\": [\"update\"], \"effect\": \"allow\" } ```

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **set** | **String**| The set | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **set** | **String**| The set | 
 **json_web_key_set** | [**JsonWebKeySet**](JsonWebKeySet.md)|  | 

### Return type

[**::models::JsonWebKeySet**](jsonWebKeySet.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

