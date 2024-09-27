"use client";

import { Inter } from "next/font/google";
import Head from "next/head";
import Script from "next/script";

// import { ThemeProvider } from "@material-tailwind/react";
import { createTheme, ThemeProvider } from "@mui/material/styles";
import CssBaseline from "@mui/material/CssBaseline";

import SessionProviderWrapper from "./components/utils/SessionProviderWrapper";
import PrelineScript from "./components/utils/PrelineScript";

import { LocalizationProvider } from "@mui/x-date-pickers";
import { AdapterDayjs } from "@mui/x-date-pickers/AdapterDayjs";

import { ApolloProvider } from "@apollo/client";
import client from "./utils/api";

import "../public/css/tailwind.css";

const inter = Inter({ subsets: ["latin"] });

const theme = createTheme({
    palette: {
        primary: {
            main: "#1976d2", // Replace with your desired primary color
        },
        secondary: {
            main: "#dc004e", // Replace with your desired secondary color
        },
        background: {
            default: "#f0f0f0", // Set background color for all pages
        },
    },
    // typography: {
    //     fontFamily: inter.className,
    // },
});

export default function RootLayout({
    children,
}: Readonly<{
    children: React.ReactNode;
}>) {
    return (
        <html lang="en">
            <Head>
                <link
                    rel="stylesheet"
                    href="https://cdn.jsdelivr.net/npm/flatpickr/dist/flatpickr.min.css"
                />
            </Head>
            <LocalizationProvider dateAdapter={AdapterDayjs}>
                <SessionProviderWrapper>
                    <ThemeProvider theme={theme}>
                        <ApolloProvider client={client}>
                            {/* <body className={inter.className}> */}
                            <body>
                                {children}
                                {/* <Script
                                src="https://cdn.jsdelivr.net/npm/flatpickr"
                                strategy="beforeInteractive"
                            />
                            <PrelineScript /> */}
                            </body>
                        </ApolloProvider>
                    </ThemeProvider>
                </SessionProviderWrapper>
            </LocalizationProvider>
        </html>
    );
}
