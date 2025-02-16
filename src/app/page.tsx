import Image from "next/image";
import HowItWorks from "@/components/OnePager/HowItWorks";
import Intro from "@/components/OnePager/Intro";
import Navbar from "@/components/OnePager/Navbar";
import Map from "@/components/OnePager/Map";
import { TracingBeam } from "@/components/ui/tracing-beam";
import ColourfulText from "@/components/ui/colourful-text";

export default function Home() {
  return (
    <div className="flex flex-col h-screen pb-20 space-y-6 px-[2%]">
      <div className="flex justify-between items-center mt-5">
        <div className="text-3xl font-bold z-2 font-sans">
          <ColourfulText text="GridX" />
        </div>
        <Navbar />
      </div>
      <Intro />
      <HowItWorks />
      <Map />
    </div>
  );
}
