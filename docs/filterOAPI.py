import yaml
import sys

# Whitelist of operationIds to retain in the OpenAPI spec
whitelist_operation_ids = [
    # Authentication
    "connectCard", # POST /auth/card
    "connectPassword", # POST /auth/password
    "logout", # GET /logout
    # Account 
    "getAccount", # GET /account
    # Categories & Items
    "getCategories", # GET /categories
    "getCategory", # GET /categories/{categoryId}
    "getCategoryPicture", # GET /categories/{categoryId}/picture
    "getCategoryItems", # GET /categories/{category_id}/items
    "getItemPicture", # GET /categories/{category_id}/items/{item_id}
]

# Whitelist of component schemas to retain in the OpenAPI spec
whitelist_schemas = [
    "UUID",
    "ErrorCodes",
    "Messages",
    "HTTPError",
    # Account
    "Account",
    "AccountRole",
    "AccountState",
    "AccountRestrictions",
    "AccountPriceRole",
    # Categories & Items
    "Category",
    "Item",
    "ItemState",
    "ItemPrices",
    "MenuItem",
    "MenuCategory",
    "Fournisseur"
]

# Whitelist of security schemes to retain in the OpenAPI spec
whitelist_security_schemes = [
    "not_onboarded",
    "auth",
    "local_token"
]

# Function to filter the OpenAPI spec by operationIds
def filter_openapi_by_operation_ids(spec, allowed_operation_ids):
    paths = spec.get('paths', {})
    new_paths = {}

    for path, methods in paths.items():
        filtered_methods = {}
        for method, operation in methods.items():
            if isinstance(operation, dict) and operation.get('operationId') in allowed_operation_ids:
                filtered_methods[method] = operation
        if filtered_methods:
            new_paths[path] = filtered_methods

    spec['paths'] = new_paths

# Function to filter component schemas by names
def filter_schemas_by_names(spec, allowed_schema_names):
    components = spec.get('components', {})
    schemas = components.get('schemas', {})
    filtered_schemas = {name: schema for name, schema in schemas.items() if name in allowed_schema_names}
    components['schemas'] = filtered_schemas
    spec['components'] = components

# Function to filter security schemes by names
def filter_security_schemes_by_names(spec, allowed_scheme_names):
    components = spec.get('components', {})
    security_schemes = components.get('securitySchemes', {})
    filtered_schemes = {name: scheme for name, scheme in security_schemes.items() if name in allowed_scheme_names}
    components['securitySchemes'] = filtered_schemes
    spec['components'] = components

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python filterOAPI.py <input_file.yaml> <output_file.yaml>")
        sys.exit(1)

    input_file = sys.argv[1]
    output_file = sys.argv[2]

    # Open the input file and parse the YAML content
    with open(input_file, 'r') as f:
        spec = yaml.safe_load(f)    

    # Filter paths based on operationIds
    filter_openapi_by_operation_ids(spec, whitelist_operation_ids)
    # Filter component schemas based on names
    filter_schemas_by_names(spec, whitelist_schemas)
    # Filter security schemes based on names

    # Save the filtered spec to the output file
    with open(output_file, 'w') as f:
        yaml.dump(spec, f, sort_keys=False)

