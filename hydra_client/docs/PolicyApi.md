# \PolicyApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_policy**](PolicyApi.md#create_policy) | **Post** /policies | Create an Access Control Policy
[**delete_policy**](PolicyApi.md#delete_policy) | **Delete** /policies/{id} | Delete an Access Control Policy
[**get_policy**](PolicyApi.md#get_policy) | **Get** /policies/{id} | Get an Access Control Policy
[**list_policies**](PolicyApi.md#list_policies) | **Get** /policies | List Access Control Policies
[**update_policy**](PolicyApi.md#update_policy) | **Put** /policies/{id} | Update an Access Control Polic


# **create_policy**
> ::models::Policy create_policy(ctx, optional)
Create an Access Control Policy

The subject making the request needs to be assigned to a policy containing:  ``` { \"resources\": [\"rn:hydra:policies\"], \"actions\": [\"create\"], \"effect\": \"allow\" } ```

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **body** | [**Policy**](Policy.md)|  | 

### Return type

[**::models::Policy**](policy.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_policy**
> delete_policy(ctx, id)
Delete an Access Control Policy

The subject making the request needs to be assigned to a policy containing:  ``` { \"resources\": [\"rn:hydra:policies:<id>\"], \"actions\": [\"delete\"], \"effect\": \"allow\" } ```

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **String**| The id of the policy. | 

### Return type

 (empty response body)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_policy**
> ::models::Policy get_policy(ctx, id)
Get an Access Control Policy

The subject making the request needs to be assigned to a policy containing:  ``` { \"resources\": [\"rn:hydra:policies:<id>\"], \"actions\": [\"get\"], \"effect\": \"allow\" } ```

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **String**| The id of the policy. | 

### Return type

[**::models::Policy**](policy.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_policies**
> Vec<::models::Policy> list_policies(ctx, optional)
List Access Control Policies

The subject making the request needs to be assigned to a policy containing:  ``` { \"resources\": [\"rn:hydra:policies\"], \"actions\": [\"list\"], \"effect\": \"allow\" } ```

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **offset** | **i64**| The offset from where to start looking. | 
 **limit** | **i64**| The maximum amount of policies returned. | 

### Return type

[**Vec<::models::Policy>**](policy.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_policy**
> ::models::Policy update_policy(ctx, id, optional)
Update an Access Control Polic

The subject making the request needs to be assigned to a policy containing:  ``` { \"resources\": [\"rn:hydra:policies\"], \"actions\": [\"update\"], \"effect\": \"allow\" } ```

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **String**| The id of the policy. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **String**| The id of the policy. | 
 **body** | [**Policy**](Policy.md)|  | 

### Return type

[**::models::Policy**](policy.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

