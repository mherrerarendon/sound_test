import 'package:flutter/material.dart';

class PartialDesc extends StatelessWidget {
  final String freq;
  final String intensity;
  const PartialDesc(this.freq, this.intensity, {Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.all(8.0),
      child: Card(
        color: Theme.of(context).colorScheme.surface,
        child: ListTile(
          leading: Icon(Icons.music_note,
              color: Theme.of(context).colorScheme.onSurface),
          title: Text(
            'Frequency: $freq',
            style: TextStyle(color: Theme.of(context).colorScheme.onSurface),
          ),
          subtitle: Text('Intensity: $intensity',
              style: TextStyle(color: Theme.of(context).colorScheme.onSurface)),
        ),
      ),
    );
  }
}
