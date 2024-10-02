export type TOMLDateTime = { $__toml_private_datetime: string }

export type WorkingHoursScheduleElement = [TOMLDateTime, TOMLDateTime]

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

export type MusicScheduleElement = [startDate: TOMLDateTime, endDate: TOMLDateTime, dirs: string[]]

export type Music = { shuffle: boolean; schedule: MusicScheduleElement[] }

export type AdvertizementSchedule = {
  [s: string]: string[]
}

export type Advertizement = {
  start_jingle: string
  end_jingle: string
  schedule?: AdvertizementSchedule
}

export type Playlist = {
  working_hours?: WorkingHours
  music?: Music
  advertizement?: Advertizement
}
