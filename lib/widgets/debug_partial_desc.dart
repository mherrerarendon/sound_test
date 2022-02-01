import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:sound_test/models/partials_model.dart';

class DebugPartialDesc extends StatelessWidget {
  const DebugPartialDesc({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final partial = Provider.of<PartialsModel>(context);
    return Padding(
      padding: const EdgeInsets.all(8.0),
      child: Card(
        color: Theme.of(context).colorScheme.surface,
        child: ListTile(
          leading: Icon(Icons.music_note,
              color: Theme.of(context).colorScheme.onSurface),
          title: Text(
            'Frequency: ${partial.freq.toStringAsFixed(2)}',
            style: TextStyle(color: Theme.of(context).colorScheme.onSurface),
          ),
          subtitle: Text('Intensity: ${partial.intensity.toStringAsFixed(2)}',
              style: TextStyle(color: Theme.of(context).colorScheme.onSurface)),
        ),
      ),
    );
  }
}
