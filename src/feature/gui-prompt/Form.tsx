import { useState } from 'react'
import { FieldInput } from './FieldInput'
import { Field } from './types'

type PromptFormProps = {
  fields: Field[]
  onSubmit: (values: Record<string, any>) => void
}

const getDefaultFormData = (fields: Field[]): Record<string, any> => {
    const data: Record<string, any> = {}
    for (const field of fields) {
      if (field.default !== undefined) {
        data[field.name] = field.default
      } else {
      switch (field.type) {
        case 'text':
          data[field.name] = ''
          break
        case 'number':
          data[field.name] = 0
          break
        default:
          data[field.name] = undefined
        }
      }
    }
    return data
}

export function Form({ fields, onSubmit }: PromptFormProps) {
  const [formData, setFormData] = useState<Record<string, any>>(() => getDefaultFormData(fields))
  const [errors, setErrors] = useState<Record<string, string | undefined>>({})

  const handleChange = (name: string, value: any) => {
    setFormData(prev => ({ ...prev, [name]: value }))
  }

  const handleSubmit = () => {
    const errors = fields.reduce((acc, field) => {
      const value = formData[field.name]
      if (field.required && (value === undefined || value === '')) {
        acc[field.name] = 'This field is required';
      } else if (typeof field.validation === 'function') {
        const result = field.validation(value)
        if (typeof result === 'string') {
          acc[field.name] = result
        }
      }
      return acc
    }, {} as Record<string, string | undefined>)

    if (Object.keys(errors).length > 0) {
      setErrors(errors)
    } else {
      setErrors({})
      onSubmit(formData)
    }
  }

  return (
    <form className="flex flex-col gap-4">
      {fields.map(field => (
        <FieldInput
          key={field.name}
          field={field}
          value={formData[field.name]}
          onChange={value => handleChange(field.name, value)}
          error={errors[field.name]}
        />
      ))}
      <button type="button" onClick={handleSubmit} className="btn btn-primary mt-4">
        Submit
      </button>
    </form>
  )
}
