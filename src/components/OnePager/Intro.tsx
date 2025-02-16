"use client";
import React from "react";
import { BackgroundBeams } from "@/components/ui/background-beams";
import Animation from "./Animation"
import ColorfulText from "./ColorfulText"

export default function BackgroundBeamsDemo() {
  return (
    <div className="min-h-screen w-full rounded-md bg-black relative flex flex-col items-center justify-center antialiased">
      {/* <div className="flex flex-col md:flex-row space-x-7 mx-auto p-4 justify-center items-center">
      <div className="mx-auto p-4">
        <h1 className="relative z-10 text-lg md:text-7xl  bg-clip-text text-transparent bg-gradient-to-b from-neutral-200 to-neutral-600  text-center font-sans font-bold">
          sell your excess energy
        </h1>
        <p></p>
        <p className="text-neutral-500 max-w-lg mx-auto my-2 text-sm text-center relative z-10">
          Welcome to MailJet, the best transactional email service on the web.
          We provide reliable, scalable, and customizable email solutions for
          your business. Whether you&apos;re sending order confirmations,
          password reset emails, or promotional campaigns, MailJet has got you
          covered.
        </p>
        <input
          type="text"
          placeholder="hi@manuarora.in"
          className="rounded-lg border border-neutral-800 focus:ring-2 focus:ring-teal-500  w-full relative z-10 mt-4  bg-neutral-950 placeholder:text-neutral-700"
        />
      </div>
      <BackgroundBeams />
       <div
            className="flex-1 w-[50%]"
            data-aos="fade-up"
            data-aos-delay="600"
          >
      <Animation/>
      </div>
      </div> */}
      <ColorfulText/>
    </div>
  );
}
