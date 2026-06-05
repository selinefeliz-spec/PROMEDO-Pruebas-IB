<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_Datos</name>
   <tag></tag>
   <elementGuidId>65e122bd-8d00-4fa8-a144-0d37879f17e7</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>#right-content-main > div</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//*[@id = 'right-content-main']/div</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorCollection>
      <entry>
         <key>SMART_LOCATOR</key>
         <value>internal:text=&quot;Datos Confirmación Validación Comprobante Su requerimiento se ha ejecutado con é&quot;i</value>
      </entry>
   </smartLocatorCollection>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
      <webElementGuid>8433e968-97e2-4288-b7f4-3430c4d5dd92</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>


    
        
            
            
                
                    
                        
                            
                        
                    
                    
                        Datos
                    
                
                
                    
                        
                            
                        
                    
                    Confirmación
                
                
                    
                        
                            
                        
                    
                    Validación
                
                
                    
                        
                            
                        
                    
                    
                        Comprobante
                    
                
            
            
        
        
    


        
            Su requerimiento se ha ejecutado con éxito.
        
        



    
        
        
            
                Enviar Comprobante
            
            
                
                    Ingrese la dirección de correo electrónico a la que desee enviar el comprobante y luego haga click en Enviar
                
                
                    
                        
                            
                                eMail:
                            
                        
                        
                            
                        
                    
                    
                         
                        
                    
                
            
            
                
                
            

            
        
    


    $('#closeButton').on(&quot;click&quot;, function () {
        $('#myModal').slideUp();
    });
    $(&quot;#sendMail_60528bac3d5449399a51c762a6504684&quot;).on(&quot;click&quot;, function (e) {
        var $frm = $('#VoucherSendEmail_60528bac3d5449399a51c762a6504684');
        if ($frm.valid()) {
            $.post('/onlinebanking/History/SendVoucherMail', $frm.serialize(), function () {
                $('#SendVoucherResult').show(500);
                $('#myModal').slideUp();
            });
            $frm.find(&quot;input[name='eMail']&quot;).val(&quot;&quot;);
            $frm.find(&quot;Email_VoucherSendEmail_60528bac3d5449399a51c762a6504684&quot;).val(&quot;&quot;);

            return true;
        }
        e.preventDefault();
        e.stopPropagation();
        return false;
    });

        
    


    

        
            
            Transacción Exitosa 
        
        
            
                
                    
                        
                            
                                
                                    
                                        
                                            
                                                Comprobante Transacción 
                                            
                                        
                                        
                                              
                                        
                                    
                                
                            
                        
                    
                
                
                    01/06/2026 12:09:48 PM
                
                
                    
                        
                            
                                
                                    Cuenta Origen:
                                    Tipo Cuenta:
                                    Alias
                                    Monto
                                    Monto de Comisión:
                                    Impuesto 0.15%:
                                
                                
                                    
                                        11011400016477
                                            DO96PRHR00000011011400016477
                                    
                                    Cuenta de Ahorro
                                    Cuenta de Ahorro
                                    RD$382.80
                                    RD$0.00
                                    RD$0.57
                                
                                 
                            

                            
                                        
                                            No. Referencia:
                                            Beneficiario
                                            
                                                    Fecha de Pago
                                            
                                            Concepto
                                            Monto
                                            Tasa de Cambio:
                                            Itbis
                                        
                                    
                                        
                                                
                                                        
                                                            99497
                                                        
                                                
                                        
                                        EDENORTE /  12345678 : No. Auth 12819 
                                        01/06/2026

                                        Pago de servicios
                                        
                                                RD$382.80
                                        
                                        
                                                No Aplica
                                        
                                            RD$0.00

                                    
                            
                        
                
            
        
    
    
        




        
            
                Comentario: Pago luz del mes
            
        
        
                    Operación sujeta al pago del impuesto de 0.15%., sobre el monto solicitado, según Norma 04/04 de la DGII.
        
    

    
   
    
                
        

 
          

     
    
    
    
        
            
                Realizar Nueva Transacción
            
        
    
</value>
      <webElementGuid>f116a764-1046-44e8-9bc5-302e9deb8dbd</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>parent</name>
      <type>Main</type>
      <value>md5.v1-c88763b6e5bb8ac96b0617d8b6ae84ea</value>
      <webElementGuid>c0e3a1bb-4305-4e89-b200-1f2c357332c2</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>//*[@id = 'right-content-main']/div</value>
      <webElementGuid>0ad59725-6aa2-42d6-ad27-78adcf45b8f3</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//*[@id = 'right-content-main']/div</value>
      <webElementGuid>c3b5787d-561e-4a30-90e0-75b05862e1df</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[(text() = concat(&quot;


    
        
            
            
                
                    
                        
                            
                        
                    
                    
                        Datos
                    
                
                
                    
                        
                            
                        
                    
                    Confirmación
                
                
                    
                        
                            
                        
                    
                    Validación
                
                
                    
                        
                            
                        
                    
                    
                        Comprobante
                    
                
            
            
        
        
    


        
            Su requerimiento se ha ejecutado con éxito.
        
        



    
        
        
            
                Enviar Comprobante
            
            
                
                    Ingrese la dirección de correo electrónico a la que desee enviar el comprobante y luego haga click en Enviar
                
                
                    
                        
                            
                                eMail:
                            
                        
                        
                            
                        
                    
                    
                         
                        
                    
                
            
            
                
                
            

            
        
    


    $(&quot; , &quot;'&quot; , &quot;#closeButton&quot; , &quot;'&quot; , &quot;).on(&quot;click&quot;, function () {
        $(&quot; , &quot;'&quot; , &quot;#myModal&quot; , &quot;'&quot; , &quot;).slideUp();
    });
    $(&quot;#sendMail_60528bac3d5449399a51c762a6504684&quot;).on(&quot;click&quot;, function (e) {
        var $frm = $(&quot; , &quot;'&quot; , &quot;#VoucherSendEmail_60528bac3d5449399a51c762a6504684&quot; , &quot;'&quot; , &quot;);
        if ($frm.valid()) {
            $.post(&quot; , &quot;'&quot; , &quot;/onlinebanking/History/SendVoucherMail&quot; , &quot;'&quot; , &quot;, $frm.serialize(), function () {
                $(&quot; , &quot;'&quot; , &quot;#SendVoucherResult&quot; , &quot;'&quot; , &quot;).show(500);
                $(&quot; , &quot;'&quot; , &quot;#myModal&quot; , &quot;'&quot; , &quot;).slideUp();
            });
            $frm.find(&quot;input[name=&quot; , &quot;'&quot; , &quot;eMail&quot; , &quot;'&quot; , &quot;]&quot;).val(&quot;&quot;);
            $frm.find(&quot;Email_VoucherSendEmail_60528bac3d5449399a51c762a6504684&quot;).val(&quot;&quot;);

            return true;
        }
        e.preventDefault();
        e.stopPropagation();
        return false;
    });

        
    


    

        
            
            Transacción Exitosa 
        
        
            
                
                    
                        
                            
                                
                                    
                                        
                                            
                                                Comprobante Transacción 
                                            
                                        
                                        
                                              
                                        
                                    
                                
                            
                        
                    
                
                
                    01/06/2026 12:09:48 PM
                
                
                    
                        
                            
                                
                                    Cuenta Origen:
                                    Tipo Cuenta:
                                    Alias
                                    Monto
                                    Monto de Comisión:
                                    Impuesto 0.15%:
                                
                                
                                    
                                        11011400016477
                                            DO96PRHR00000011011400016477
                                    
                                    Cuenta de Ahorro
                                    Cuenta de Ahorro
                                    RD$382.80
                                    RD$0.00
                                    RD$0.57
                                
                                 
                            

                            
                                        
                                            No. Referencia:
                                            Beneficiario
                                            
                                                    Fecha de Pago
                                            
                                            Concepto
                                            Monto
                                            Tasa de Cambio:
                                            Itbis
                                        
                                    
                                        
                                                
                                                        
                                                            99497
                                                        
                                                
                                        
                                        EDENORTE /  12345678 : No. Auth 12819 
                                        01/06/2026

                                        Pago de servicios
                                        
                                                RD$382.80
                                        
                                        
                                                No Aplica
                                        
                                            RD$0.00

                                    
                            
                        
                
            
        
    
    
        




        
            
                Comentario: Pago luz del mes
            
        
        
                    Operación sujeta al pago del impuesto de 0.15%., sobre el monto solicitado, según Norma 04/04 de la DGII.
        
    

    
   
    
                
        

 
          

     
    
    
    
        
            
                Realizar Nueva Transacción
            
        
    
&quot;) or . = concat(&quot;


    
        
            
            
                
                    
                        
                            
                        
                    
                    
                        Datos
                    
                
                
                    
                        
                            
                        
                    
                    Confirmación
                
                
                    
                        
                            
                        
                    
                    Validación
                
                
                    
                        
                            
                        
                    
                    
                        Comprobante
                    
                
            
            
        
        
    


        
            Su requerimiento se ha ejecutado con éxito.
        
        



    
        
        
            
                Enviar Comprobante
            
            
                
                    Ingrese la dirección de correo electrónico a la que desee enviar el comprobante y luego haga click en Enviar
                
                
                    
                        
                            
                                eMail:
                            
                        
                        
                            
                        
                    
                    
                         
                        
                    
                
            
            
                
                
            

            
        
    


    $(&quot; , &quot;'&quot; , &quot;#closeButton&quot; , &quot;'&quot; , &quot;).on(&quot;click&quot;, function () {
        $(&quot; , &quot;'&quot; , &quot;#myModal&quot; , &quot;'&quot; , &quot;).slideUp();
    });
    $(&quot;#sendMail_60528bac3d5449399a51c762a6504684&quot;).on(&quot;click&quot;, function (e) {
        var $frm = $(&quot; , &quot;'&quot; , &quot;#VoucherSendEmail_60528bac3d5449399a51c762a6504684&quot; , &quot;'&quot; , &quot;);
        if ($frm.valid()) {
            $.post(&quot; , &quot;'&quot; , &quot;/onlinebanking/History/SendVoucherMail&quot; , &quot;'&quot; , &quot;, $frm.serialize(), function () {
                $(&quot; , &quot;'&quot; , &quot;#SendVoucherResult&quot; , &quot;'&quot; , &quot;).show(500);
                $(&quot; , &quot;'&quot; , &quot;#myModal&quot; , &quot;'&quot; , &quot;).slideUp();
            });
            $frm.find(&quot;input[name=&quot; , &quot;'&quot; , &quot;eMail&quot; , &quot;'&quot; , &quot;]&quot;).val(&quot;&quot;);
            $frm.find(&quot;Email_VoucherSendEmail_60528bac3d5449399a51c762a6504684&quot;).val(&quot;&quot;);

            return true;
        }
        e.preventDefault();
        e.stopPropagation();
        return false;
    });

        
    


    

        
            
            Transacción Exitosa 
        
        
            
                
                    
                        
                            
                                
                                    
                                        
                                            
                                                Comprobante Transacción 
                                            
                                        
                                        
                                              
                                        
                                    
                                
                            
                        
                    
                
                
                    01/06/2026 12:09:48 PM
                
                
                    
                        
                            
                                
                                    Cuenta Origen:
                                    Tipo Cuenta:
                                    Alias
                                    Monto
                                    Monto de Comisión:
                                    Impuesto 0.15%:
                                
                                
                                    
                                        11011400016477
                                            DO96PRHR00000011011400016477
                                    
                                    Cuenta de Ahorro
                                    Cuenta de Ahorro
                                    RD$382.80
                                    RD$0.00
                                    RD$0.57
                                
                                 
                            

                            
                                        
                                            No. Referencia:
                                            Beneficiario
                                            
                                                    Fecha de Pago
                                            
                                            Concepto
                                            Monto
                                            Tasa de Cambio:
                                            Itbis
                                        
                                    
                                        
                                                
                                                        
                                                            99497
                                                        
                                                
                                        
                                        EDENORTE /  12345678 : No. Auth 12819 
                                        01/06/2026

                                        Pago de servicios
                                        
                                                RD$382.80
                                        
                                        
                                                No Aplica
                                        
                                            RD$0.00

                                    
                            
                        
                
            
        
    
    
        




        
            
                Comentario: Pago luz del mes
            
        
        
                    Operación sujeta al pago del impuesto de 0.15%., sobre el monto solicitado, según Norma 04/04 de la DGII.
        
    

    
   
    
                
        

 
          

     
    
    
    
        
            
                Realizar Nueva Transacción
            
        
    
&quot;))]</value>
      <webElementGuid>c641d68a-808d-4768-b135-d02a90b0cfe1</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
