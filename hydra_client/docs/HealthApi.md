# \HealthApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_instance_status**](HealthApi.md#get_instance_status) | **Get** /health/status | Check the Health Status


# **get_instance_status**
> Value get_instance_status()
Check the Health Status

This endpoint returns a 200 status code when the HTTP server is up running. `{ \"status\": \"ok\" }`. This status does currently not include checks whether the database connection is working. This endpoint does not require the `X-Forwarded-Proto` header when TLS termination is set.  Be aware that if you are running multiple nodes of ORY Hydra, the health status will never refer to the cluster state, only to a single instance.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**Value**](Value.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

