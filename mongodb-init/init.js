
const config =
{
    service_endpoints: {
        grant_request_endpoint: "http://localhost:8000/gnap/tx",
        introspection_endpoint: "http://localhost:8000/gnap/introspect",
        resource_registration_endpoint: "http://localhost:8000/gnap/resource",
    },
    interaction_start_modes_supported: [
        "redirect",
        "app",
        "user_code"
    ],
    interaction_finish_methods_supported: [
        "redirect",
        "push"
    ],
    key_proofs_supported: [
        "httpsig",
        "mtls",
        "jwsd",
        "jws"
    ],
    subject_formats_supported: [
        "account",
        "aliases",
        "did",
        "email",
        "iss_sub",
        "opaque",
        "phone_number"
    ],
    assertions_supported: [
        "oidc",
        "saml2"
    ],
    token_formats_supported: [
        "jwt",
        "paseto"
    ]
};

const clients = [
    {
        client_id: "7e057b0c-17e8-4ab4-9260-2b33f32b2cce",
        client_name: "test_client_1",
        redirect_uris: ["http://localhost:8000"]
    }
]

const users = [
]


const accounts = [
    {
        account_id: 'e63769de-3a44-11ec-8d3d-0242ac130003',
        address: [
            {
                country: '000',
                formatted: '000',
                locality: '000',
                postal_code: '000',
                region: '000',
                street_address: '000',
            }
        ],
        birthdate: '1987-10-16',
        email: [
            {
                address: 'johndoe@example.com',
                verified: false,
                primary: true,
            },
        ],
        family_name: 'Doe',
        gender: 'male',
        given_name: 'John',
        locale: 'en-US',
        middle_name: 'Middle',
        name: 'John Doe',
        nickname: 'Johny',
        phone: [
            {
                phone_number: '+49 000 000000',
                verified: false,
                primary: true,
            },
        ],
        picture: 'http://lorempixel.com/400/200/people',
        preferred_username: 'johnny',
        profile: 'https://johnswebsite.com',
        website: 'http://example.com',
        zoneinfo: 'Europe/Berlin',
    },
    {
        account_id: 'e63769de-3a44-11ec-8d3d-0242ac130001',
        address: [
            {
                country: '000',
                formatted: '000',
                locality: '000',
                postal_code: '000',
                region: '000',
                street_address: '000',
            }
        ],
        birthdate: '1983-01-29',
        email: [
            {
                address: 'ken@kefo.no',
                verified: false,
                primary: true,
            },
        ],
        family_name: 'Fossen',
        gender: 'male',
        given_name: 'Kenneth',
        locale: 'en-US',
        middle_name: '',
        name: 'Kenneth Fossen',
        nickname: 'spydx',
        phone: [
            {
                phone_number: '+47 0000 0000',
                verified: false,
                primary: true,
            },
        ],
        picture: 'http://lorempixel.com/400/200/people',
        preferred_username: 'spydx',
        profile: 'https://profile.kefo.no',
        website: 'https://kefo.no',
        zoneinfo: 'Europe/Berlin',
    },
]

const resources = [

]

const tokens = [

]
let conn = new Mongo();
db = conn.getDB("gnap");
db.service_config.insert(config);
db.clients.insertMany(clients);
db.accounts.insertMany(accounts);
db.tokens.insertMany(tokens);
db.resources.insertMany(resources);