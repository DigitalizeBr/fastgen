from pytest_bdd import scenarios, given, when, then

scenarios('../features/example.feature')

@given("eu tenho os números 2 e 3")
def input_numeros():
    return (2, 3)

@when("eu somo os dois")
def somar(input_numeros):
    a, b = input_numeros
    return a + b

@then("o resultado deve ser 5")
def resultado(somar):
    assert somar == 5
