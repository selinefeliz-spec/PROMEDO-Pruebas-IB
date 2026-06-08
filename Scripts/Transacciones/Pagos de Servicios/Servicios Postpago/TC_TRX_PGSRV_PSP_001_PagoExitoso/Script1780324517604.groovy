import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import org.openqa.selenium.WebDriver as WebDriver

WebUI.callTestCase(findTestCase('TC_LoginExitoso'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Page_Banco Promerica - Banca en Linea/div_Transacciones'))

WebUI.click(findTestObject('Page_Banco Promerica - Banca en Linea/span_Pagos de Servicios'))

WebUI.selectOptionByValue(findTestObject('Page_Banco Promerica - Banca en Linea/select_AccountFromStr'), '-26805417191', 
    false)

WebUI.setText(findTestObject('Page_Banco Promerica - Banca en Linea/textarea_trx-comment'), 'Pago luz del mes')

WebUI.click(findTestObject('Page_Banco Promerica - Banca en Linea/input_btn btn-primary'))

WebUI.click(findTestObject('Page_Banco Promerica - Banca en Linea/input_frmContinue'))

WebUI.setText(findTestObject('Page_Banco Promerica - Banca en Linea/input_ChallengesRequests_0_Challenges_0_SIICHKAn'), 
    GlobalVariable.verifyCode)

WebUI.click(findTestObject('Page_Banco Promerica - Banca en Linea/input_btn btn-primary_1'))

WebUI.waitForElementVisible(findTestObject('Page_Banco Promerica - Banca en Linea/Page_Banco Promerica - Banca en Linea/div_Datos'), 
    10)

WebUI.click(findTestObject('Page_Banco Promerica - Banca en Linea/i_icon-envelope icon-2x'))

if (false) {
    WebUI.verifyElementVisible(findTestObject('Page_Banco Promerica - Banca en Linea/InfoShowDialogs/ErrorShowDialogs/div_Ha ocurrido un error efectuando una ope_39c5f7'))

    WebUI.click(findTestObject('button_Cerrar'))
} else {
    WebUI.setText(findTestObject('Page_Banco Promerica - Banca en Linea/input_Email_VoucherSendEmail_02b9770e37d241b28ca'), 
        findTestData('Lista_Corresos_PROMEDO').getValue(1, 1))

    WebUI.click(findTestObject('Page_Banco Promerica - Banca en Linea/input_sendMail_02b9770e37d241b28cabe00ece6a0bd0'))

    WebUI.click(findTestObject('Page_Banco Promerica - Banca en Linea/img_img-logout'))
}

