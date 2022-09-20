# UpdateWindow

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**length** | Option<**String**> | Sets the length of the update window beginning with the start time. This needs to be a valid duration as parsed by Go's time.ParseDuration (https://pkg.go.dev/time#ParseDuration), e.g. `2h`. | [optional]
**start** | Option<**String**> | Sets the start time of the update window. This can be a time of day in 24h format, e.g. `22:30`, or a day of week plus a time of day, for example `Mon 21:00`. Only short names for week days are supported, i.e. `Mon`, `Tue`, `Wed`, `Thu`, `Fri`, `Sat` and `Sun`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


