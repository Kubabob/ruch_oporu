import folium

m = folium.Map(location=[52.570, 19.248], zoom_start=12)

folium.Marker(
    location=[53.1319, 23.1595],
    tooltip="Otwarty parasol",
    popup="Otwarty parasol",
    icon=folium.Icon(icon="umbrella"),
).add_to(m)

m.save("map.html")
