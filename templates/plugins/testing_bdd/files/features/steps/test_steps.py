from pytest_bdd import scenarios, given, when, then

scenarios('../example.feature')

@given("eu tenho os números 2 e 3", target_fixture="input_numeros")
def input_numeros():
    return (2, 3)

@when("eu somo os dois", target_fixture="somar")
def somar(input_numeros):
    a, b = input_numeros
    return a + b

@then("o resultado deve ser 5")
def resultado(somar):
    assert somar == 5
