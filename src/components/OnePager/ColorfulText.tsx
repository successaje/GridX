"use client";
import React from "react";
import ColourfulText from "@/components/ui/colourful-text";
import { motion } from "motion/react";

export default function ColourfulTextDemo() {
  return (
    <div className="h-screen w-full flex items-center justify-center relative overflow-hidden bg-black">
      <motion.img
        src="grid.jpg"
        className="h-full w-full object-cover absolute inset-0 [mask-image:radial-gradient(circle,transparent,black_80%)] pointer-events-none"
        initial={{ opacity: 0 }}
        animate={{ opacity: 0.5 }}
        transition={{ duration: 1 }}
      />
      <h1 className="text-2xl md:text-3xl lg:text-7xl font-bold text-center text-white relative z-2 font-sans">
        Powering a decentralized future with blockchain-based{" "}
        <ColourfulText text=" peer-to-peer" /> <br />
        energy trading.
      </h1>
    </div>
  );
}
