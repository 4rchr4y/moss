import { useTranslation } from 'react-i18next'
import { commands } from '../bindings'
import React, { useEffect, useState } from 'react'
import { listen } from '@tauri-apps/api/event'

export const Home: React.FC = () => {
  const { t } = useTranslation(['ns1', 'ns2'])
  const [name, setName] = useState('')
  const [data, setData] = useState<number | null>(null)

  useEffect(() => {
    const fetchName = async () => {
      try {
        await commands.createProject('test')
        const response = await commands.greet('g10z3r')
        setName(response)
      } catch (error) {
        console.error('Failed to fetch greeting:', error)
      }
    }

    fetchName()

    const unlisten = listen<number>('data-stream', (event) => {
      setData(event.payload)
    })

    return () => {
      unlisten.then((f) => f())
    }
  }, [])

  return (
    <main>
      <h1>{t('title')}</h1>
      {name && <p>{t('user', { name })}</p>}
      <span>{t('description.part1')}</span>
      <span>{t('description.part1', { ns: 'ns2' })}</span>
      {data !== null && <p>Received data: {data}</p>}
    </main>
  )
}
