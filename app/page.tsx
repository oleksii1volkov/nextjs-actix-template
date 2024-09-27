"use client";

import React from "react";
import NavBar from "./components/mui/NavBar";
import TourCard from "./components/material/TourCard";
import TourSearch from "./components/TourSearch";

import { useEffect, useState } from "react";
import { fetchTours } from "./utils/apiEndpoints";

const Home: React.FC = () => {
    const [tours, setTours] = useState<any[]>([]);
    const [loading, setLoading] = useState<boolean>(true);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        const loadData = async () => {
            try {
                const fetchedTours = await fetchTours();
                setTours(fetchedTours);
                setLoading(false);
            } catch (error) {
                console.error("Error fetching tours:", error);
                setError("Failed to load tours.");
                setLoading(false);
            }
        };

        loadData();
    }, []);

    return (
        <>
            <div className="pb-6">
                <NavBar />
            </div>

            <div className="p-24">
                <TourSearch />
            </div>

            <div className="container mx-auto px-4">
                <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
                    {loading ? (
                        <p>Loading tours...</p>
                    ) : error ? (
                        <p>{error}</p>
                    ) : (
                        tours.map((tour) => (
                            <div key={tour.id} className="">
                                <TourCard />
                            </div>
                        ))
                    )}
                </div>
            </div>
        </>
    );
};

export default Home;
