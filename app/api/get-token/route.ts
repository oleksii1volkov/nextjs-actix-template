import { getToken } from "next-auth/jwt";
import { NextRequest, NextResponse } from "next/server";

const secret = process.env.NEXTAUTH_SECRET;

// export async function GET(req) {
//     const token = await getToken({ req, secret });

//     if (token) {
//         // Signed in
//         console.log("JSON Web Token", JSON.stringify(token, null, 2));
//         return NextResponse.json({ token });
//     } else {
//         // Not Signed in
//         return new NextResponse("Unauthorized", { status: 401 });
//     }
// }

export async function GET(request: NextRequest) {
    const token = await getToken({ req: request, secret });
    const backendUrl = "http://localhost:8000"; // Replace with your backend URL

    // Forward the request to the backend
    const response = await fetch(backendUrl, {
        method: request.method,
        headers: request.headers,
        // If your request has a body, you might want to include it:
        // body: req.body,
    });

    // Read the response from the backend
    const data = await response.text();

    console.log(data);

    // Create a new response with the same status and headers as the backend response
    return new Response(data, {
        status: response.status,
        headers: response.headers,
    });
}
