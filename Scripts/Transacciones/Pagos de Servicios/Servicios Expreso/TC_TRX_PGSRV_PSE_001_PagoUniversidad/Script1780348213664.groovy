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

WebUI.callTestCase(findTestCase('TC_LoginExitoso'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.switchToWindowTitle('Banco Promerica - Banca en Linea')

WebUI.click(findTestObject('div_Transacciones'))

WebUI.click(findTestObject('span_Pagos de Servicios'))

WebUI.click(findTestObject('a_expressServices'))

WebUI.selectOptionByValue(findTestObject('select_AccountFromStr'), '-26805417191', false)

WebUI.selectOptionByValue(findTestObject('select_Categoria'), 'UNIVERSITY', false)

WebUI.selectOptionByValue(findTestObject('select_Empresa _'), '40', false)

WebUI.setText(findTestObject('input_PayExpressContractNumber'), '1123456789')

WebUI.click(findTestObject('button_validateExpress'))

WebUI.click(findTestObject('i_icon-eye-open'))

WebUI.assertElementPresent(findTestObject('Page_Banco Promerica - Banca en Linea/InfoShowDialogs/div_payee-info-modal-body'), 
    0)

WebUI.click(findTestObject('button_Cerrar'))

WebUI.click(findTestObject('input_Otro Monto'))

WebUI.click(findTestObject('input_AnyAmountPayment'))

WebUI.setText(findTestObject('input_AnyAmountPayment'), '500')

WebUI.click(findTestObject('Page_Banco Promerica - Banca en Linea/textarea_trx-comment'))

WebUI.setText(findTestObject('textarea_trx-comment'), 'Pago universidad ')

WebUI.click(findTestObject('input_btn btn-primary'))

WebUI.click(findTestObject('input_frmContinue'))

WebUI.setText(findTestObject('input_ChallengesRequests_0_Challenges_0_SIICHKAn'), '')

WebUI.setText(findTestObject('input_ChallengesRequests_0_Challenges_0_SIICHKAn'), '123456')

WebUI.click(findTestObject('input_btn btn-primary_1'))

if (true) {
    WebUI.assertElementVisible(findTestObject('Page_Banco Promerica - Banca en Linea/InfoShowDialogs/ErrorShowDialogs/div_Estimado cliente en este momento no podemos'), 
        0)

    WebUI.click(findTestObject('button_Cerrar'))
} else {
    WebUI.click(findTestObject('Page_Banco Promerica - Banca en Linea/i_icon-envelope icon-2x'))
}

