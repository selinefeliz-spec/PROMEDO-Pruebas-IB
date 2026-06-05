<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_Personal                                                    Empresarial</name>
   <tag></tag>
   <elementGuidId>e0a0a912-e578-4b0f-9a96-1fe07e1f8e5e</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>#main-content-body</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//div[@id='main-content-body']</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorCollection>
      <entry>
         <key>SMART_LOCATOR</key>
         <value>#main-content-body</value>
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
      <webElementGuid>91a1945c-12b2-4922-859c-5cb37573386f</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>id</name>
      <type>Main</type>
      <value>main-content-body</value>
      <webElementGuid>ba3bc2fb-d05f-44cc-8e0a-a6d49c159151</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>row body-cont layout-main-content-body</value>
      <webElementGuid>8acadc0f-15e7-442f-b493-58678c0e5f18</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>

        
            
                







    
        
            
            Personal

        
    
    
    
        
            
            Empresarial
        
    


    
            
                
                    
                    
                
                
            
            
                
                    
                    
                
                
            
            
                
                    Iniciar Sesión
                
            
            
                
                    Solicitar Nuevo Producto
                
            
    
    
            
                
                    
                    
                
                
            
            
                
                    
                    
                
                
            
            
                
                    
                    
                
                
            
            
                
                    Iniciar Sesión
                
            
    




    function setRecapchaPersonal() {
        grecaptcha.ready(function () {
            grecaptcha.execute('6Lf8x94UAAAAAOS4DVDdHH8ssYwJowTmJKNxEfCw', { action: &quot;homepage_login_personal&quot; }).then(function (token) {
                $(&quot;#captchaTokenPersonal&quot;).val(token);
            });
        });
    }

    function setRecapchaEnterprise() {
        grecaptcha.ready(function () {
            grecaptcha.execute('6Lf8x94UAAAAAOS4DVDdHH8ssYwJowTmJKNxEfCw', { action: &quot;homepage_login_enterprise&quot; }).then(function (token) {
                $(&quot;#captchaTokenEnteprise&quot;).val(token);
            });
        });
    }

    $(document).ready(function () {
        $(&quot;.fpd&quot;).val(getMachineInformation());

        var fingerprintJsonGenerated = false;

        function shouldCallFingerprintScript() {
            return typeof window !== 'undefined' &amp;&amp;
                typeof SII !== 'undefined' &amp;&amp; typeof SII.Fingerprint !== 'undefined';
        }

        function generateAndSetFingerprintJson() {
            if (!fingerprintJsonGenerated &amp;&amp; shouldCallFingerprintScript()) {
                SII.Fingerprint.generateFingerprintJson().then(function(fingerprint) {
                    var jsonString = JSON.stringify(fingerprint);
                    $(&quot;#fingerprintJsonP&quot;).val(jsonString);
                    $(&quot;#fingerprintJsonE&quot;).val(jsonString);
                    fingerprintJsonGenerated = true;
                }).catch(function(error) {
                    console.warn('Error al generar fingerprint JSON:', error);
                    fingerprintJsonGenerated = true;
                });
            } else {
                fingerprintJsonGenerated = true;
            }
        }

        generateAndSetFingerprintJson();

        $(&quot;#LoginFormPersonal, #LoginFormEnterprise&quot;).on('submit', function (e) {
            if (!fingerprintJsonGenerated) {
                e.preventDefault();
                generateAndSetFingerprintJson();
                setTimeout(function () {
                    $(e.target).submit();
                }, 500);
                return false;
            }
        });
    });

    function LogOnClearForm() {
        $(&quot;input[type='password']&quot;).val(&quot;&quot;);
        $(&quot;.UserName&quot;).focus();

    }

                
                    
                        Olvide mi contraseña
                    
                    
                    
                        Necesita ayuda
                    
                
                
                    
                        Consejos de Seguridad
                    
                    
                    
                        Browsers soportados
                    
                
                
                    
                        Registrarme en Banco Promerica 
                    
                

            

            
            

                



    // Set the banners
    var banner = []

    banner.push(&quot;&lt;p>&lt;img  alt='https://firebasestorage.googleapis.com/v0/b/promerica-mbapp.appspot.com/o/marketing%2FBanner-PromericaOnline.jpg?alt=media&amp;token=7ef03c70-159b-46c4-8538-d706a3015bce' src='https://firebasestorage.googleapis.com/v0/b/promerica-mbapp.appspot.com/o/marketing%2FBanner-PromericaOnline.jpg?alt=media&amp;token=7ef03c70-159b-46c4-8538-d706a3015bce' class='img-responsive' />&lt;/p>&quot;);
    

    banner.push(&quot;&lt;p>&lt;img  alt='https://firebasestorage.googleapis.com/v0/b/siidevpush.appspot.com/o/Promerica-Online_Banner.jpg?alt=media&amp;token=37ce2470-03cd-4341-84b1-f54dc3c85015' src='https://firebasestorage.googleapis.com/v0/b/siidevpush.appspot.com/o/Promerica-Online_Banner.jpg?alt=media&amp;token=37ce2470-03cd-4341-84b1-f54dc3c85015' class='img-responsive' />&lt;/p>&quot;);
    

    // Banners on windows load
    //window.onload = function () { setOwlBanners(banner); }
    $(document).ready(function () { setOwlBanners(banner); });

    function setOwlBanners(banners, selector) {
        var __selector = selector || '#AdRotator';
        var rotator = $(__selector);
        // Check that we have banners
        // Check that the cycle plugin exist
        // and check that a container exist
        //if (!banners || banners.length === 0
        //    &amp;&amp; typeof $.fn.cycle === 'function'
        //    &amp;&amp; rotator.length > 0) { return; }

        var imgWrapper = '&lt;div class=&quot;item&quot;>';
        content = imgWrapper;
        content += banners.join('&lt;/div>' + imgWrapper);
        content += &quot;&lt;/div>&quot;;
        rotator.html(content);
        var bnrs = rotator.find(&quot;img&quot;);
        //bnrs.addClass('img-responsive');
        bnrs.removeAttr('style');
        var atLeastOneLoaded = false;

        bnrs.one(&quot;load&quot;, function () {
            if (!atLeastOneLoaded) {
                atLeastOneLoaded = true;
                $('.owl-carousel').owlCarousel({
                    loop: true,
                    items: 1,
                    lazyLoad: true,
                    autoplayHoverPause: true,
                    autoHeight: false, 
                    autoplay: true
                });
            }
        }).each(function () {
            if (this.complete) $(this).load();
        });

    }

                
                
                    
                        
                        
                            ¡Síguenos!
                            
                                
                            
                            
                                
                            
                            
                                
                            
                            
                                
                            
                            
                                
                            
                        
                    
                     
                    
                        
                        
                            Tel.:809.732.6006servicio@promerica.com.do
                        
                    
                     
                    
                        
                            
                                
                                Entidad Financiera Autorizada
                            
                        
                    
                
            
        
        
            
                Descarga nuestra app
                
                    
                
                
                    
                
            
        
    </value>
      <webElementGuid>1e2fe4dc-4d6a-4ef6-aa0d-1ee80ab23494</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;main-content-body&quot;)</value>
      <webElementGuid>d9ba16f2-fa77-468e-afdd-945d2488ed05</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//div[@id='main-content-body']</value>
      <webElementGuid>2f5c6d81-b00a-4835-9b17-15018f1c8eed</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Su Navegador no soporta el Tag Video'])[1]/following::div[19]</value>
      <webElementGuid>3bcdf82e-7185-4dc9-b6fe-51f557e79517</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Consulta Tasa de Cambio'])[1]/following::div[50]</value>
      <webElementGuid>4a68db00-958d-46e6-a48e-08e8c37d8961</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div[15]</value>
      <webElementGuid>83c822a6-02d0-405a-83a9-c2768e6f2ab9</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[@id = 'main-content-body' and (text() = concat(&quot;

        
            
                







    
        
            
            Personal

        
    
    
    
        
            
            Empresarial
        
    


    
            
                
                    
                    
                
                
            
            
                
                    
                    
                
                
            
            
                
                    Iniciar Sesión
                
            
            
                
                    Solicitar Nuevo Producto
                
            
    
    
            
                
                    
                    
                
                
            
            
                
                    
                    
                
                
            
            
                
                    
                    
                
                
            
            
                
                    Iniciar Sesión
                
            
    




    function setRecapchaPersonal() {
        grecaptcha.ready(function () {
            grecaptcha.execute(&quot; , &quot;'&quot; , &quot;6Lf8x94UAAAAAOS4DVDdHH8ssYwJowTmJKNxEfCw&quot; , &quot;'&quot; , &quot;, { action: &quot;homepage_login_personal&quot; }).then(function (token) {
                $(&quot;#captchaTokenPersonal&quot;).val(token);
            });
        });
    }

    function setRecapchaEnterprise() {
        grecaptcha.ready(function () {
            grecaptcha.execute(&quot; , &quot;'&quot; , &quot;6Lf8x94UAAAAAOS4DVDdHH8ssYwJowTmJKNxEfCw&quot; , &quot;'&quot; , &quot;, { action: &quot;homepage_login_enterprise&quot; }).then(function (token) {
                $(&quot;#captchaTokenEnteprise&quot;).val(token);
            });
        });
    }

    $(document).ready(function () {
        $(&quot;.fpd&quot;).val(getMachineInformation());

        var fingerprintJsonGenerated = false;

        function shouldCallFingerprintScript() {
            return typeof window !== &quot; , &quot;'&quot; , &quot;undefined&quot; , &quot;'&quot; , &quot; &amp;&amp;
                typeof SII !== &quot; , &quot;'&quot; , &quot;undefined&quot; , &quot;'&quot; , &quot; &amp;&amp; typeof SII.Fingerprint !== &quot; , &quot;'&quot; , &quot;undefined&quot; , &quot;'&quot; , &quot;;
        }

        function generateAndSetFingerprintJson() {
            if (!fingerprintJsonGenerated &amp;&amp; shouldCallFingerprintScript()) {
                SII.Fingerprint.generateFingerprintJson().then(function(fingerprint) {
                    var jsonString = JSON.stringify(fingerprint);
                    $(&quot;#fingerprintJsonP&quot;).val(jsonString);
                    $(&quot;#fingerprintJsonE&quot;).val(jsonString);
                    fingerprintJsonGenerated = true;
                }).catch(function(error) {
                    console.warn(&quot; , &quot;'&quot; , &quot;Error al generar fingerprint JSON:&quot; , &quot;'&quot; , &quot;, error);
                    fingerprintJsonGenerated = true;
                });
            } else {
                fingerprintJsonGenerated = true;
            }
        }

        generateAndSetFingerprintJson();

        $(&quot;#LoginFormPersonal, #LoginFormEnterprise&quot;).on(&quot; , &quot;'&quot; , &quot;submit&quot; , &quot;'&quot; , &quot;, function (e) {
            if (!fingerprintJsonGenerated) {
                e.preventDefault();
                generateAndSetFingerprintJson();
                setTimeout(function () {
                    $(e.target).submit();
                }, 500);
                return false;
            }
        });
    });

    function LogOnClearForm() {
        $(&quot;input[type=&quot; , &quot;'&quot; , &quot;password&quot; , &quot;'&quot; , &quot;]&quot;).val(&quot;&quot;);
        $(&quot;.UserName&quot;).focus();

    }

                
                    
                        Olvide mi contraseña
                    
                    
                    
                        Necesita ayuda
                    
                
                
                    
                        Consejos de Seguridad
                    
                    
                    
                        Browsers soportados
                    
                
                
                    
                        Registrarme en Banco Promerica 
                    
                

            

            
            

                



    // Set the banners
    var banner = []

    banner.push(&quot;&lt;p>&lt;img  alt=&quot; , &quot;'&quot; , &quot;https://firebasestorage.googleapis.com/v0/b/promerica-mbapp.appspot.com/o/marketing%2FBanner-PromericaOnline.jpg?alt=media&amp;token=7ef03c70-159b-46c4-8538-d706a3015bce&quot; , &quot;'&quot; , &quot; src=&quot; , &quot;'&quot; , &quot;https://firebasestorage.googleapis.com/v0/b/promerica-mbapp.appspot.com/o/marketing%2FBanner-PromericaOnline.jpg?alt=media&amp;token=7ef03c70-159b-46c4-8538-d706a3015bce&quot; , &quot;'&quot; , &quot; class=&quot; , &quot;'&quot; , &quot;img-responsive&quot; , &quot;'&quot; , &quot; />&lt;/p>&quot;);
    

    banner.push(&quot;&lt;p>&lt;img  alt=&quot; , &quot;'&quot; , &quot;https://firebasestorage.googleapis.com/v0/b/siidevpush.appspot.com/o/Promerica-Online_Banner.jpg?alt=media&amp;token=37ce2470-03cd-4341-84b1-f54dc3c85015&quot; , &quot;'&quot; , &quot; src=&quot; , &quot;'&quot; , &quot;https://firebasestorage.googleapis.com/v0/b/siidevpush.appspot.com/o/Promerica-Online_Banner.jpg?alt=media&amp;token=37ce2470-03cd-4341-84b1-f54dc3c85015&quot; , &quot;'&quot; , &quot; class=&quot; , &quot;'&quot; , &quot;img-responsive&quot; , &quot;'&quot; , &quot; />&lt;/p>&quot;);
    

    // Banners on windows load
    //window.onload = function () { setOwlBanners(banner); }
    $(document).ready(function () { setOwlBanners(banner); });

    function setOwlBanners(banners, selector) {
        var __selector = selector || &quot; , &quot;'&quot; , &quot;#AdRotator&quot; , &quot;'&quot; , &quot;;
        var rotator = $(__selector);
        // Check that we have banners
        // Check that the cycle plugin exist
        // and check that a container exist
        //if (!banners || banners.length === 0
        //    &amp;&amp; typeof $.fn.cycle === &quot; , &quot;'&quot; , &quot;function&quot; , &quot;'&quot; , &quot;
        //    &amp;&amp; rotator.length > 0) { return; }

        var imgWrapper = &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;item&quot;>&quot; , &quot;'&quot; , &quot;;
        content = imgWrapper;
        content += banners.join(&quot; , &quot;'&quot; , &quot;&lt;/div>&quot; , &quot;'&quot; , &quot; + imgWrapper);
        content += &quot;&lt;/div>&quot;;
        rotator.html(content);
        var bnrs = rotator.find(&quot;img&quot;);
        //bnrs.addClass(&quot; , &quot;'&quot; , &quot;img-responsive&quot; , &quot;'&quot; , &quot;);
        bnrs.removeAttr(&quot; , &quot;'&quot; , &quot;style&quot; , &quot;'&quot; , &quot;);
        var atLeastOneLoaded = false;

        bnrs.one(&quot;load&quot;, function () {
            if (!atLeastOneLoaded) {
                atLeastOneLoaded = true;
                $(&quot; , &quot;'&quot; , &quot;.owl-carousel&quot; , &quot;'&quot; , &quot;).owlCarousel({
                    loop: true,
                    items: 1,
                    lazyLoad: true,
                    autoplayHoverPause: true,
                    autoHeight: false, 
                    autoplay: true
                });
            }
        }).each(function () {
            if (this.complete) $(this).load();
        });

    }

                
                
                    
                        
                        
                            ¡Síguenos!
                            
                                
                            
                            
                                
                            
                            
                                
                            
                            
                                
                            
                            
                                
                            
                        
                    
                     
                    
                        
                        
                            Tel.:809.732.6006servicio@promerica.com.do
                        
                    
                     
                    
                        
                            
                                
                                Entidad Financiera Autorizada
                            
                        
                    
                
            
        
        
            
                Descarga nuestra app
                
                    
                
                
                    
                
            
        
    &quot;) or . = concat(&quot;

        
            
                







    
        
            
            Personal

        
    
    
    
        
            
            Empresarial
        
    


    
            
                
                    
                    
                
                
            
            
                
                    
                    
                
                
            
            
                
                    Iniciar Sesión
                
            
            
                
                    Solicitar Nuevo Producto
                
            
    
    
            
                
                    
                    
                
                
            
            
                
                    
                    
                
                
            
            
                
                    
                    
                
                
            
            
                
                    Iniciar Sesión
                
            
    




    function setRecapchaPersonal() {
        grecaptcha.ready(function () {
            grecaptcha.execute(&quot; , &quot;'&quot; , &quot;6Lf8x94UAAAAAOS4DVDdHH8ssYwJowTmJKNxEfCw&quot; , &quot;'&quot; , &quot;, { action: &quot;homepage_login_personal&quot; }).then(function (token) {
                $(&quot;#captchaTokenPersonal&quot;).val(token);
            });
        });
    }

    function setRecapchaEnterprise() {
        grecaptcha.ready(function () {
            grecaptcha.execute(&quot; , &quot;'&quot; , &quot;6Lf8x94UAAAAAOS4DVDdHH8ssYwJowTmJKNxEfCw&quot; , &quot;'&quot; , &quot;, { action: &quot;homepage_login_enterprise&quot; }).then(function (token) {
                $(&quot;#captchaTokenEnteprise&quot;).val(token);
            });
        });
    }

    $(document).ready(function () {
        $(&quot;.fpd&quot;).val(getMachineInformation());

        var fingerprintJsonGenerated = false;

        function shouldCallFingerprintScript() {
            return typeof window !== &quot; , &quot;'&quot; , &quot;undefined&quot; , &quot;'&quot; , &quot; &amp;&amp;
                typeof SII !== &quot; , &quot;'&quot; , &quot;undefined&quot; , &quot;'&quot; , &quot; &amp;&amp; typeof SII.Fingerprint !== &quot; , &quot;'&quot; , &quot;undefined&quot; , &quot;'&quot; , &quot;;
        }

        function generateAndSetFingerprintJson() {
            if (!fingerprintJsonGenerated &amp;&amp; shouldCallFingerprintScript()) {
                SII.Fingerprint.generateFingerprintJson().then(function(fingerprint) {
                    var jsonString = JSON.stringify(fingerprint);
                    $(&quot;#fingerprintJsonP&quot;).val(jsonString);
                    $(&quot;#fingerprintJsonE&quot;).val(jsonString);
                    fingerprintJsonGenerated = true;
                }).catch(function(error) {
                    console.warn(&quot; , &quot;'&quot; , &quot;Error al generar fingerprint JSON:&quot; , &quot;'&quot; , &quot;, error);
                    fingerprintJsonGenerated = true;
                });
            } else {
                fingerprintJsonGenerated = true;
            }
        }

        generateAndSetFingerprintJson();

        $(&quot;#LoginFormPersonal, #LoginFormEnterprise&quot;).on(&quot; , &quot;'&quot; , &quot;submit&quot; , &quot;'&quot; , &quot;, function (e) {
            if (!fingerprintJsonGenerated) {
                e.preventDefault();
                generateAndSetFingerprintJson();
                setTimeout(function () {
                    $(e.target).submit();
                }, 500);
                return false;
            }
        });
    });

    function LogOnClearForm() {
        $(&quot;input[type=&quot; , &quot;'&quot; , &quot;password&quot; , &quot;'&quot; , &quot;]&quot;).val(&quot;&quot;);
        $(&quot;.UserName&quot;).focus();

    }

                
                    
                        Olvide mi contraseña
                    
                    
                    
                        Necesita ayuda
                    
                
                
                    
                        Consejos de Seguridad
                    
                    
                    
                        Browsers soportados
                    
                
                
                    
                        Registrarme en Banco Promerica 
                    
                

            

            
            

                



    // Set the banners
    var banner = []

    banner.push(&quot;&lt;p>&lt;img  alt=&quot; , &quot;'&quot; , &quot;https://firebasestorage.googleapis.com/v0/b/promerica-mbapp.appspot.com/o/marketing%2FBanner-PromericaOnline.jpg?alt=media&amp;token=7ef03c70-159b-46c4-8538-d706a3015bce&quot; , &quot;'&quot; , &quot; src=&quot; , &quot;'&quot; , &quot;https://firebasestorage.googleapis.com/v0/b/promerica-mbapp.appspot.com/o/marketing%2FBanner-PromericaOnline.jpg?alt=media&amp;token=7ef03c70-159b-46c4-8538-d706a3015bce&quot; , &quot;'&quot; , &quot; class=&quot; , &quot;'&quot; , &quot;img-responsive&quot; , &quot;'&quot; , &quot; />&lt;/p>&quot;);
    

    banner.push(&quot;&lt;p>&lt;img  alt=&quot; , &quot;'&quot; , &quot;https://firebasestorage.googleapis.com/v0/b/siidevpush.appspot.com/o/Promerica-Online_Banner.jpg?alt=media&amp;token=37ce2470-03cd-4341-84b1-f54dc3c85015&quot; , &quot;'&quot; , &quot; src=&quot; , &quot;'&quot; , &quot;https://firebasestorage.googleapis.com/v0/b/siidevpush.appspot.com/o/Promerica-Online_Banner.jpg?alt=media&amp;token=37ce2470-03cd-4341-84b1-f54dc3c85015&quot; , &quot;'&quot; , &quot; class=&quot; , &quot;'&quot; , &quot;img-responsive&quot; , &quot;'&quot; , &quot; />&lt;/p>&quot;);
    

    // Banners on windows load
    //window.onload = function () { setOwlBanners(banner); }
    $(document).ready(function () { setOwlBanners(banner); });

    function setOwlBanners(banners, selector) {
        var __selector = selector || &quot; , &quot;'&quot; , &quot;#AdRotator&quot; , &quot;'&quot; , &quot;;
        var rotator = $(__selector);
        // Check that we have banners
        // Check that the cycle plugin exist
        // and check that a container exist
        //if (!banners || banners.length === 0
        //    &amp;&amp; typeof $.fn.cycle === &quot; , &quot;'&quot; , &quot;function&quot; , &quot;'&quot; , &quot;
        //    &amp;&amp; rotator.length > 0) { return; }

        var imgWrapper = &quot; , &quot;'&quot; , &quot;&lt;div class=&quot;item&quot;>&quot; , &quot;'&quot; , &quot;;
        content = imgWrapper;
        content += banners.join(&quot; , &quot;'&quot; , &quot;&lt;/div>&quot; , &quot;'&quot; , &quot; + imgWrapper);
        content += &quot;&lt;/div>&quot;;
        rotator.html(content);
        var bnrs = rotator.find(&quot;img&quot;);
        //bnrs.addClass(&quot; , &quot;'&quot; , &quot;img-responsive&quot; , &quot;'&quot; , &quot;);
        bnrs.removeAttr(&quot; , &quot;'&quot; , &quot;style&quot; , &quot;'&quot; , &quot;);
        var atLeastOneLoaded = false;

        bnrs.one(&quot;load&quot;, function () {
            if (!atLeastOneLoaded) {
                atLeastOneLoaded = true;
                $(&quot; , &quot;'&quot; , &quot;.owl-carousel&quot; , &quot;'&quot; , &quot;).owlCarousel({
                    loop: true,
                    items: 1,
                    lazyLoad: true,
                    autoplayHoverPause: true,
                    autoHeight: false, 
                    autoplay: true
                });
            }
        }).each(function () {
            if (this.complete) $(this).load();
        });

    }

                
                
                    
                        
                        
                            ¡Síguenos!
                            
                                
                            
                            
                                
                            
                            
                                
                            
                            
                                
                            
                            
                                
                            
                        
                    
                     
                    
                        
                        
                            Tel.:809.732.6006servicio@promerica.com.do
                        
                    
                     
                    
                        
                            
                                
                                Entidad Financiera Autorizada
                            
                        
                    
                
            
        
        
            
                Descarga nuestra app
                
                    
                
                
                    
                
            
        
    &quot;))]</value>
      <webElementGuid>cdf4d13c-853e-42c8-996f-146d6f8ec176</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
