import { About, Content, DraggableTopBar, Home, Menu, Properties, RootLayout, Sidebar } from '@/components'
import { Suspense, useState } from 'react'
import { BrowserRouter, Route, Routes } from 'react-router-dom'
import './i18n'

function App() {
  return (
    <>
      <DraggableTopBar />
      <RootLayout>
        <Sidebar className="p-2 border-2 border-red-500">Sidebar</Sidebar>
        <Content className="border-2 border-blue-500">
          <Suspense fallback="loading">
            <BrowserRouter>
              <Menu />
              <Routes>
                <Route path="/" element={<Home />} />
                <Route path="/about" element={<About />} />
              </Routes>
            </BrowserRouter>
          </Suspense>
        </Content>
        <Properties className="p-2 border-2 border-green-500">P</Properties>
      </RootLayout>
    </>
  )
}

export default App
