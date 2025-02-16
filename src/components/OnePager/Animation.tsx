"use client";
import React from "react";
import Lottie from "lottie-react";
import ICON from "@/GIF/lottie.json";

function Animation() {
  return <Lottie animationData={ICON} loop={true} />;
}

export default Animation;
