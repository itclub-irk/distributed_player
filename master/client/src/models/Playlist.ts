export type TOMLTime = { $__toml_private_datetime: string }

export type WorkingHoursScheduleElement = [TOMLTime, TOMLTime]

export type WorkingHoursException = { [d: string]: WorkingHoursScheduleElement }

export type WorkingHoursSchedule = [
  WorkingHoursScheduleElement,
  WorkingHoursScheduleElement,
  WorkingHoursScheduleElement,
  WorkingHoursScheduleElement,
  WorkingHoursScheduleElement,
  WorkingHoursScheduleElement,
  WorkingHoursScheduleElement
]

export type WorkingHours = { exceptions: WorkingHoursException; schedule: WorkingHoursSchedule }

export type Playlist = { working_hours?: WorkingHours }
