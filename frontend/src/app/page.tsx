'use client';

import { useState, useEffect } from 'react';
import { useWebSocket } from '@/hooks/useWebSocket';

export default function Home() {
  const [prompt, setPrompt] = useState('');
  const { lastMessage, sendMessage, isConnected } = useWebSocket('ws://127.0.0.1:3001/ws');

  // Debounced prompt sending
  useEffect(() => {
    const timer = setTimeout(() => {
      if (prompt) {
        sendMessage(prompt);
      }
    }, 200); // 200ms debounce

    return () => clearTimeout(timer);
  }, [prompt, sendMessage]);

  return (
    <main className="relative flex min-h-screen flex-col items-center justify-center overflow-hidden bg-black text-white">
      {/* Real-time Canvas Area */}
      <div className="absolute inset-0 flex items-center justify-center">
        <div className="relative h-full w-full max-w-4xl p-4 md:p-8 flex items-center justify-center">
          <div className="relative w-full aspect-square bg-zinc-900 rounded-2xl border border-zinc-800 shadow-2xl flex items-center justify-center overflow-hidden">
            {lastMessage && lastMessage.startsWith('data:image') ? (
              <>
                <img 
                  src={lastMessage} 
                  alt="Generated" 
                  className={`w-full h-full object-cover transition-all duration-700 ${
                    isGenerating ? 'opacity-40 scale-105 blur-sm' : 'opacity-100 scale-100 blur-0'
                  }`}
                />
                {isGenerating && (
                  <div className="absolute inset-0 flex items-center justify-center">
                    <div className="w-12 h-12 border-t-2 border-blue-500 rounded-full animate-spin shadow-[0_0_15px_rgba(59,130,246,0.5)]"></div>
                  </div>
                )}
              </>
            ) : lastMessage ? (
              <div className="text-center p-8">
                <p className="text-zinc-400 text-sm mb-2">Message:</p>
                <p className="text-xl font-medium text-zinc-100">{lastMessage}</p>
              </div>
            ) : (
              <div className="text-zinc-600 animate-pulse">
                Ready to generate...
              </div>
            )}
            
            {/* Status indicator */}
            <div className="absolute top-4 right-4 flex items-center gap-2 px-3 py-1 rounded-full bg-black/50 border border-white/10 backdrop-blur-sm">
              <div className={`h-2 w-2 rounded-full ${isConnected ? 'bg-green-500 shadow-[0_0_8px_rgba(34,197,94,0.6)]' : 'bg-red-500'}`} />
              <span className="text-[10px] font-medium tracking-wider uppercase text-zinc-400">
                {isConnected ? 'Live' : 'Disconnected'}
              </span>
            </div>
          </div>
        </div>
      </div>

      {/* Timeline Parallel (Phase 4) */}
      <div className="absolute top-24 right-8 w-48 bottom-48 overflow-y-auto pr-4 scrollbar-hide flex flex-col gap-4">
        {history.map((item) => (
          <button
            key={item.id}
            onClick={() => restoreFromHistory(item)}
            className="relative aspect-square w-full rounded-xl border border-white/5 bg-zinc-900 overflow-hidden hover:border-blue-500/50 transition-all group"
          >
            <img src={item.image} alt={item.prompt} className="w-full h-full object-cover opacity-60 group-hover:opacity-100 transition-opacity" />
            <div className="absolute inset-0 bg-gradient-to-t from-black/80 via-transparent to-transparent opacity-0 group-hover:opacity-100 transition-opacity flex items-end p-2">
              <p className="text-[8px] text-zinc-300 truncate w-full">{item.prompt}</p>
            </div>
          </button>
        ))}
      </div>

      {/* Floating UI Layer (Phase 3 foundation) */}
      <div className="absolute bottom-12 w-full max-w-2xl px-6 flex flex-col gap-4">
        {/* Advanced Settings Panel */}
        <div 
          className={`bg-zinc-900/90 backdrop-blur-2xl border border-white/10 rounded-2xl shadow-2xl overflow-hidden transition-all duration-500 ease-in-out ${
            showAdvanced ? 'max-h-96 opacity-100 p-6 mb-2' : 'max-h-0 opacity-0 p-0 mb-0'
          }`}
        >
          <div className="grid grid-cols-2 gap-8">
            <div className="space-y-3">
              <label className="text-[10px] uppercase tracking-widest text-zinc-500 font-bold">Seed</label>
              <input 
                type="number" 
                value={seed} 
                onChange={(e) => setSeed(Number(e.target.value))}
                className="w-full bg-black/40 border border-white/5 rounded-lg px-4 py-2 text-sm focus:border-blue-500/50 outline-none transition-colors"
              />
            </div>
            <div className="space-y-3">
              <label className="text-[10px] uppercase tracking-widest text-zinc-500 font-bold">Steps</label>
              <input 
                type="range" 
                min="1" max="50" 
                value={steps} 
                onChange={(e) => setSteps(Number(e.target.value))}
                className="w-full accent-blue-500"
              />
              <div className="flex justify-between text-[10px] text-zinc-600 font-mono">
                <span>1</span>
                <span>{steps}</span>
                <span>50</span>
              </div>
            </div>
          </div>
        </div>

        {/* Main Prompt Bar */}
        <div className="relative group">
          <div className="absolute -inset-1 bg-gradient-to-r from-blue-600 to-purple-600 rounded-2xl blur opacity-20 group-focus-within:opacity-40 transition duration-500"></div>
          <div className="relative bg-zinc-900/80 backdrop-blur-xl border border-white/10 rounded-2xl shadow-2xl overflow-hidden">
            <input
              type="text"
              value={prompt}
              onChange={(e) => setPrompt(e.target.value)}
              placeholder="Describe something magical..."
              className="w-full bg-transparent px-6 py-5 text-lg outline-none placeholder:text-zinc-600"
              autoFocus
            />
            
            {/* Minimal controls hint */}
            <div className="flex items-center justify-between px-6 py-2 border-t border-white/5 bg-black/20">
              <div className="flex gap-4">
                <button className="text-[10px] text-zinc-500 hover:text-zinc-300 transition-colors uppercase tracking-widest font-bold">
                  Style: Cinematic
                </button>
              </div>
              <button 
                onClick={() => setShowAdvanced(!showAdvanced)}
                className={`text-[10px] transition-colors uppercase tracking-widest font-bold ${
                  showAdvanced ? 'text-blue-400' : 'text-zinc-500 hover:text-zinc-300'
                }`}
              >
                {showAdvanced ? 'Hide Settings' : 'Advanced Settings'}
              </button>
            </div>
          </div>
        </div>
      </div>

      {/* Brand Hint */}
      <div className="absolute top-8 left-8">
        <h1 className="text-xl font-light tracking-[0.2em] uppercase text-white/40">
          Nebula <span className="text-white/10 font-black">Canvas</span>
        </h1>
      </div>
    </main>
  );
}
