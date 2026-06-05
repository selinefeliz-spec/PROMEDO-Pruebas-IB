<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST_USER</name>
   <tag></tag>
   <elementGuidId>2206fe10-a9e0-41a4-8109-dd59deaf07bc</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;AppId\&quot;: \&quot;28963205-1c13-4b3b-ac35-b2f680de9345\&quot;,\n    \&quot;Password\&quot;: \&quot;qwerty123123\&quot;,\n    \&quot;deviceID\&quot;: 3388748197,\n    \&quot;deviceModel\&quot;: \&quot;Chrome\&quot;,\n    \&quot;applicationVersion\&quot;: \&quot;122.0.0.0\&quot;,\n    \&quot;deviceType\&quot;: \&quot;unknown-Chrome\&quot;,\n    \&quot;OSVersion\&quot;: \&quot;browser\&quot;,\n    \&quot;phoneNumber\&quot;: \&quot;\&quot;,\n    \&quot;UserName\&quot;: \&quot;camilo\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>49bb4076-5b0e-4db8-9cf1-14204f3218f9</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>11.2.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://promedo:8022/auth/api/auth/init</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
