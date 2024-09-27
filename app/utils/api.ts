import {
    ApolloClient,
    InMemoryCache,
    HttpLink,
    ApolloLink,
} from "@apollo/client";
import { onError } from "@apollo/client/link/error";

// Create an Apollo Link for adding request interceptors
const requestLink = new ApolloLink((operation, forward) => {
    // Add the authorization token or any other headers
    operation.setContext(({ headers = {} }) => ({
        headers: {
            ...headers,
            // Example: Add Authorization token
            // Authorization: `Bearer ${localStorage.getItem('token')}`,
        },
    }));

    console.log("Sending request...");

    return forward(operation);
});

// Create an Apollo Link for handling errors
const errorLink = onError(({ graphQLErrors, networkError }) => {
    if (graphQLErrors) {
        graphQLErrors.forEach(({ message, locations, path }) => {
            console.error(
                `[GraphQL error]: Message: ${message}, Location: ${locations}, Path: ${path}`
            );
        });
    }

    if (networkError) {
        console.error(`[Network error]: ${networkError}`);
    }
});

// Create the Apollo Client instance with the links
const client = new ApolloClient({
    link: ApolloLink.from([
        requestLink,
        errorLink,
        new HttpLink({ uri: "http://localhost:8000/graphql" }),
    ]),
    cache: new InMemoryCache(),
});

export default client;
