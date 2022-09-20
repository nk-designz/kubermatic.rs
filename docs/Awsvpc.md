# Awsvpc

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cidr_block** | Option<**String**> | The primary IPv4 CIDR block for the VPC. | [optional]
**cidr_block_association_set** | Option<[**Vec<crate::models::AwsVpcCidrBlockAssociation>**](AWSVpcCidrBlockAssociation.md)> | Information about the IPv4 CIDR blocks associated with the VPC. | [optional]
**dhcp_options_id** | Option<**String**> | The ID of the set of DHCP options you've associated with the VPC (or default if the default options are associated with the VPC). | [optional]
**instance_tenancy** | Option<**String**> | The allowed tenancy of instances launched into the VPC. | [optional]
**ipv6_cidr_block_association_set** | Option<[**Vec<crate::models::AwsVpcIpv6CidrBlockAssociation>**](AWSVpcIpv6CidrBlockAssociation.md)> | Information about the IPv6 CIDR blocks associated with the VPC. | [optional]
**is_default** | Option<**bool**> | Indicates whether the VPC is the default VPC. | [optional]
**name** | Option<**String**> |  | [optional]
**owner_id** | Option<**String**> | The ID of the AWS account that owns the VPC. | [optional]
**state** | Option<**String**> | The current state of the VPC. | [optional]
**tags** | Option<[**Vec<crate::models::AwsTag>**](AWSTag.md)> | Any tags assigned to the VPC. | [optional]
**vpc_id** | Option<**String**> | The ID of the VPC. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


