import { gql } from "@apollo/client";
import client from "./api";

// Define the GET_TOURS query
const GET_TOURS = gql`
    query GetTours {
        getTours {
            id
            title
            description
            startDate
            endDate
            price
            rating
            createdAt
            updatedAt
            location
            imageUrl
            isActive
            maxParticipants
        }
    }
`;

// Define the CREATE_TOUR mutation
const CREATE_TOUR = gql`
    mutation CreateTour(
        $title: String!
        $description: String
        $start_date: String
        $end_date: String
        $price: Float
        $rating: Float
        $location: String
        $image_url: String
        $is_active: Boolean!
        $max_participants: Int
    ) {
        createTour(
            title: $title
            description: $description
            start_date: $start_date
            end_date: $end_date
            price: $price
            rating: $rating
            location: $location
            image_url: $image_url
            is_active: $is_active
            max_participants: $max_participants
        ) {
            id
            title
            created_at
            updated_at
        }
    }
`;

// Fetch tours function
export const fetchTours = async () => {
    try {
        const { data } = await client.query({ query: GET_TOURS });
        console.log("Received tours: ", data.getTours);
        return data.getTours; // Ensure this is an array
    } catch (error) {
        console.error("Error fetching tours:", error);
        throw error;
    }
};

/*
// Create tour function
export const createTour = async (tourInput) => {
    try {
        const { data } = await client.mutate({
            mutation: CREATE_TOUR,
            variables: tourInput,
        });
        console.log("Created tour: ", data.createTour);
        return data.createTour;
    } catch (error) {
        console.error("Error creating tour:", error);
        throw error;
    }
};
*/
