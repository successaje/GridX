import Image from "next/image";
import HowItWorks from "@/components/OnePager/HowItWorks"
import Intro from "@/components/OnePager/Intro"
import Navbar from "@/components/OnePager/Navbar"
import Map from "@/components/OnePager/Map"


export default function Home() {
  return (
    <div className="flex flex-col items-center justify-items-center min-h-screen  pb-20 gap-16 sm:p-20 font-[family-name:var(--font-geist-sans)]">
    <Navbar/>
    <Intro/>
    <HowItWorks/>
    <Map/>
    </div>
  );
}
