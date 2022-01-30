import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:sound_test/models/partials_model.dart';

class MyStatefulWidget extends StatefulWidget {
  const MyStatefulWidget({Key? key}) : super(key: key);

  @override
  State<MyStatefulWidget> createState() => _MyStatefulWidgetState();
}

class _MyStatefulWidgetState extends State<MyStatefulWidget>
    with SingleTickerProviderStateMixin {
  late final AnimationController _controller = AnimationController(
    duration: const Duration(milliseconds: 100),
    vsync: this,
  );
  late final Animation<Offset> _offsetAnimation =
      Tween<Offset>(begin: Offset.zero, end: Offset.zero).animate(_controller);

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Consumer<PartialsModel>(builder: (context, partials, _) {
      scrollTo() {
        final centsOffset = partials.centsOffset / 100;

        _controller.animateTo(centsOffset,
            duration: const Duration(milliseconds: 100), curve: Curves.linear);
      }

      WidgetsBinding.instance?.addPostFrameCallback((_) => scrollTo());
      return LayoutBuilder(builder: (context, constraints) {
        final midX = constraints.maxWidth / 2;
        final offset = partials.centsOffset / 100 * constraints.maxWidth + midX;
        return Stack(
          children: [
            Positioned(
                child: SizedBox(
                    child: FittedBox(
                        child: Column(
                      children: [
                        Icon(Icons.arrow_upward),
                        Text(
                            '${partials.centsOffset > 0 ? '+' : ''}${partials.centsOffset.toStringAsFixed(2)}')
                      ],
                    )),
                    width: 50,
                    height: constraints.maxHeight),
                left: offset - 25),
          ],
        );
      });
    });
  }
}

class PitchPointer extends StatelessWidget {
  const PitchPointer({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Consumer<PartialsModel>(
      builder: (context, partials, _) {
        final centsOffset = partials.centsOffset;
        return Text(
            'Cents offset: ${centsOffset > 0 ? '+' : ''}${centsOffset.toStringAsFixed(2)}',
            style: TextStyle(
                fontSize: 20,
                color: partials.inTune()
                    ? Theme.of(context).colorScheme.onSurface
                    : centsOffset < 0
                        ? Colors.red
                        : Colors.blue));
      },
    );
  }
}
